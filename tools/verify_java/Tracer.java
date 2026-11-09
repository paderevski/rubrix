import com.sun.jdi.*;
import com.sun.jdi.connect.*;
import com.sun.jdi.event.*;
import com.sun.jdi.request.*;
import java.io.*;
import java.util.*;

/**
 * Runs a compiled class under the JDI debugger and prints a JSON trace:
 * one entry per executed line of the target class, with local variable state
 * captured just before that line runs, plus final state and program stdout.
 *
 * usage: java --add-modules jdk.jdi Tracer.java <classDir> <mainClass> [maxSteps]
 */
public class Tracer {
    static String esc(String s) {
        StringBuilder b = new StringBuilder("\"");
        for (char c : s.toCharArray()) {
            switch (c) {
                case '"': b.append("\\\""); break;
                case '\\': b.append("\\\\"); break;
                case '\n': b.append("\\n"); break;
                case '\r': b.append("\\r"); break;
                case '\t': b.append("\\t"); break;
                default: if (c < 0x20) b.append(String.format("\\u%04x", (int) c)); else b.append(c);
            }
        }
        return b.append('"').toString();
    }

    static String show(Value v) {
        if (v == null) return "null";
        if (v instanceof StringReference) return "\"" + ((StringReference) v).value() + "\"";
        if (v instanceof CharValue) return "'" + ((CharValue) v).value() + "'";
        if (v instanceof ArrayReference) {
            List<Value> vals = ((ArrayReference) v).getValues();
            StringJoiner j = new StringJoiner(", ", "[", "]");
            for (Value e : vals) j.add(show(e));
            return j.toString();
        }
        if (v instanceof ObjectReference) {
            ObjectReference o = (ObjectReference) v;
            String t = o.referenceType().name();
            if (t.equals("java.util.ArrayList")) {
                int n = ((IntegerValue) o.getValue(o.referenceType().fieldByName("size"))).value();
                if (n == 0) return "[]";
                ArrayReference data = (ArrayReference) o.getValue(o.referenceType().fieldByName("elementData"));
                StringJoiner j = new StringJoiner(", ", "[", "]");
                for (Value e : data.getValues(0, n)) j.add(show(e));
                return j.toString();
            }
            if (t.startsWith("java.lang.") && !t.equals("java.lang.Object")) {
                Field f = o.referenceType().fieldByName("value");
                if (f != null) return show(o.getValue(f));
            }
            return "<" + t.substring(t.lastIndexOf('.') + 1) + ">";
        }
        return v.toString();
    }

    static String locals(StackFrame f) throws Exception {
        StringJoiner j = new StringJoiner(",", "{", "}");
        try {
            for (LocalVariable lv : f.visibleVariables())
                j.add(esc(lv.name()) + ":" + esc(show(f.getValue(lv))));
        } catch (AbsentInformationException e) { /* compiled without -g */ }
        return j.toString();
    }

    public static void main(String[] a) throws Exception {
        String dir = a[0], main = a[1];
        int max = a.length > 2 ? Integer.parseInt(a[2]) : 5000;

        LaunchingConnector lc = Bootstrap.virtualMachineManager().defaultConnector();
        Map<String, Connector.Argument> args = lc.defaultArguments();
        args.get("main").setValue(main);
        args.get("options").setValue("-cp " + dir);
        VirtualMachine vm = lc.launch(args);

        StringBuilder out = new StringBuilder();
        Thread t = new Thread(() -> {
            try (Reader r = new InputStreamReader(vm.process().getInputStream())) {
                char[] buf = new char[4096]; int n;
                while ((n = r.read(buf)) > 0) synchronized (out) { out.append(buf, 0, n); }
            } catch (IOException ignored) {}
        });
        t.start();
        StringBuilder err = new StringBuilder();
        Thread te = new Thread(() -> {
            try (Reader r = new InputStreamReader(vm.process().getErrorStream())) {
                char[] buf = new char[4096]; int n;
                while ((n = r.read(buf)) > 0) synchronized (err) { err.append(buf, 0, n); }
            } catch (IOException ignored) {}
        });
        te.start();

        ClassPrepareRequest cpr = vm.eventRequestManager().createClassPrepareRequest();
        cpr.addClassFilter(main);
        cpr.enable();

        List<String> steps = new ArrayList<>();
        String lastLocals = "{}";
        boolean truncated = false;
        boolean done = false;
        while (!done) {
            EventSet es = vm.eventQueue().remove();
            for (Event e : es) {
                if (e instanceof ClassPrepareEvent) {
                    ThreadReference th = ((ClassPrepareEvent) e).thread();
                    StepRequest sr = vm.eventRequestManager().createStepRequest(th, StepRequest.STEP_LINE, StepRequest.STEP_INTO);
                    sr.addClassFilter(main);
                    sr.enable();
                    MethodExitRequest mer = vm.eventRequestManager().createMethodExitRequest();
                    mer.addClassFilter(main);
                    mer.enable();
                } else if (e instanceof StepEvent) {
                    StepEvent se = (StepEvent) e;
                    if (steps.size() >= max) { truncated = true; vm.exit(1); done = true; break; }
                    String l = locals(se.thread().frame(0));
                    steps.add("{\"line\":" + se.location().lineNumber() + ",\"method\":" + esc(se.location().method().name()) + ",\"before\":" + l + "}");
                } else if (e instanceof MethodExitEvent) {
                    MethodExitEvent me = (MethodExitEvent) e;
                    if (me.method().name().equals("main")) lastLocals = locals(me.thread().frame(0));
                } else if (e instanceof VMDeathEvent || e instanceof VMDisconnectEvent) {
                    done = true;
                }
            }
            if (!done) es.resume();
        }
        t.join(2000); te.join(2000);
        synchronized (out) {
            System.out.println("{\"stdout\":" + esc(out.toString()) + ",\"stderr\":" + esc(err.toString())
                + ",\"truncated\":" + truncated + ",\"final\":" + lastLocals + ",\"steps\":[" + String.join(",", steps) + "]}");
        }
    }
}
