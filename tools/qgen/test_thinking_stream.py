#!/usr/bin/env python3
"""Does Claude Haiku 5.5 stream readable reasoning text, and what does each setting do to tokens and latency?

Sends the app's real prompt (rendered by bank.py, same as the offline experiments) once per variant and prints, per variant:
what the stream looked like (thinking chunks / chars, time to first thinking and first answer text), the token usage,
and the first part of any reasoning text. Variants a server rejects are reported, not fatal.

Run it yourself so the key stays in your shell:
  pip install anthropic
  ! ANTHROPIC_API_KEY=sk-... python3 tools/qgen/test_thinking_stream.py [--topic T006] [--difficulty medium] [--show 1500]
Cost: about $0.003 per variant.
"""
import argparse, os, random, sys, time
from pathlib import Path

import anthropic

sys.path.insert(0, str(Path(__file__).parent))
import bank  # noqa: E402

MODEL = "claude-haiku-5-5"
VARIANTS = [
    ("default (no thinking param)", {}),
    ("adaptive, display summarized", {"thinking": {"type": "adaptive", "display": "summarized"}}),
    ("adaptive, display omitted", {"thinking": {"type": "adaptive", "display": "omitted"}}),
    ("summarized + effort low", {"thinking": {"type": "adaptive", "display": "summarized"}, "output_config": {"effort": "low"}}),
    ("summarized + effort high", {"thinking": {"type": "adaptive", "display": "summarized"}, "output_config": {"effort": "high"}}),
    ("thinking disabled + effort low", {"thinking": {"type": "disabled"}, "output_config": {"effort": "low"}}),
]


def run(client, prompt, extra, show):
    t0 = time.time()
    th, tx, t_think, t_text = [], [], None, None
    with client.messages.stream(model=MODEL, max_tokens=16000, system="You are a helpful assistant.",
                                messages=[{"role": "user", "content": prompt}], **extra) as stream:
        for ev in stream:
            if ev.type == "content_block_delta":
                if ev.delta.type == "thinking_delta":
                    t_think = t_think or time.time() - t0
                    th.append(ev.delta.thinking)
                elif ev.delta.type == "text_delta":
                    t_text = t_text or time.time() - t0
                    tx.append(ev.delta.text)
        u = stream.get_final_message().usage
    thinking = "".join(th)
    f = lambda x: "-" if x is None else f"{x:.1f}s"
    print(f"   thinking chunks {len(th)}, chars {len(thinking)}; first thinking {f(t_think)}, first answer text {f(t_text)}, total {time.time() - t0:.1f}s")
    print(f"   usage: input {u.input_tokens}, output {u.output_tokens} (output includes thinking), answer chars {len(''.join(tx))}")
    if thinking:
        print("   reasoning text >>", thinking[:show].replace("\n", "\n      "))
    else:
        print("   reasoning text: (empty)")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--topic", default="T006")
    ap.add_argument("--difficulty", default="medium", choices=["easy", "medium", "hard"])
    ap.add_argument("--show", type=int, default=1500, help="characters of reasoning text to print")
    a = ap.parse_args()
    if not os.environ.get("ANTHROPIC_API_KEY"):
        sys.exit("set ANTHROPIC_API_KEY in your shell (this script never stores it)")
    subj = bank.Subject()
    ex = subj.select_examples([a.topic], a.difficulty, 3, random.Random(1))
    prompt = subj.baseline_prompt([a.topic], a.difficulty, 1, ex)
    client = anthropic.Anthropic()
    for name, extra in VARIANTS:
        print(f"\n== {name}")
        try:
            run(client, prompt, extra, a.show)
        except anthropic.APIStatusError as e:
            print(f"   REJECTED ({e.status_code}): {str(e.message)[:200]}")
        except Exception as e:
            print(f"   error: {e}")


if __name__ == "__main__":
    main()
