# Arrays and ArrayList Test 1 - Version 1

::: {custom-style="QuestionBody"}
**Question 1.** The method below is intended to return the **last** index at which the given `target` string appears in `list`. If the target does not appear, the method should return `-1`.
:::

```java
public static int lastIndexOfTarget(ArrayList<String> list, String target) {
    for (int i = 0; i < list.size(); i++) {
        if (list.get(i).equals(target)) {
            return i; // supposed to find the last occurrence
        }
    }
    return -1;
}
```
Which **single** change will make the method work as described?

(Assume the rest of the program is correct.)

A) Change the increment to `i--` but keep the initialization `int i = 0`.
B) Negate the comparison: `if (!list.get(i).equals(target)) return i;`.
C) Change the loop header to `for (int i = list.size() - 1; i >= 0; i--)`.
D) Change the loop condition to `i <= list.size() - 1`.

::: {custom-style="QuestionBody"}
**Question 2.** The code below creates an `ArrayList<String>` named `words`.
:::

```java
ArrayList<String> words = new ArrayList<>();
words.add("alpha");
words.add("beta");
words.add("gamma");
words.add("delta");
```
Which of the following code fragments, placed after the statements above, will print the elements of `words` in reverse order (one per line) without using `Collections.reverse` or creating a second list?

Select the **single** fragment that works correctly for any list size.

A)
    ```java
    for (int i = words.size(); i > 0; i--) {
        System.out.println(words.get(i));
    }
    ```
B)
    ```java
    int i = words.size();
    while (i-- > 1) {
        System.out.println(words.get(i));
    }
    ```
C)
    ```java
    for (int i = words.size() - 1; i >= 0; i--) {
        System.out.println(words.get(i));
    }
    ```
D)
    ```java
    for (String s : words) {
        System.out.println(s);
    }
    ```
E)
    ```java
    for (int i = 0; i < words.size(); i++) {
        System.out.println(words.get(i));
    }
    ```

::: {custom-style="QuestionBody"}
**Question 3.** The method below is intended to return the average of the values in `values` as a `double`. However, it usually returns an incorrect value. Which of the following can fix the problem?
:::

```java
public static double average(int[] values) {
    int total = 0;
    for (int v : values) {
        total += v;
    }
    return total / values.length;
}
```
Which single change will make the method return the correct average as a `double`?
Select the best modification.

A) Change the loop header to `for (int i = 0; i <= values.length; i++) total += values[i];`.
B) Change the return statement to `return (double) total / values.length;`.
C) Change the method’s return type from `double` to `int`.
D) Change the return statement to `return total / (values.length - 1);`.

::: {custom-style="QuestionBody"}
**Question 4.** The following code creates an `ArrayList<Integer>` named `numbers`:
:::

```java
ArrayList<Integer> numbers = new ArrayList<>();
numbers.add(1);
numbers.add(2);
numbers.add(4);
numbers.add(5);
numbers.add(6);
numbers.add(7);
numbers.add(8);
numbers.add(9);
numbers.add(10);
```
One of the code fragments below is inserted after these statements. After the fragment executes, `numbers` must contain **only the odd integers that were originally in the list** (i.e., `[1, 5, 7, 9]`) and the program must **not throw any exception**. Which fragment satisfies the requirement?

**A)**
```java
for (int n : numbers) {
    if (n % 2 == 0) {
        numbers.remove(Integer.valueOf(n));
    }
}
```

**B)**
```java
for (int i = 0; i < numbers.size(); i++) {
    if (numbers.get(i) % 2 == 0) {
        numbers.remove(i);
    }
}
```

**C)**
```java
for (int i = numbers.size(); i >= 0; i--) {
    if (numbers.get(i) % 2 == 0) {
        numbers.remove(i);
    }
}
```

**D)**
```java
for (int i = numbers.size() - 1; i >= 0; i--) {
    if (numbers.get(i) % 2 == 0) {
        numbers.remove(i);
    }
}
```

A) C
B) B
C) D
D) A

::: {custom-style="QuestionBody"}
**Question 5.** Consider the following code segment.
:::

```java
int[] data = {3, 5, 2, 8, 6};
for (int i = 0; i < data.length; i++) {
    data[i] = data[i] * 2;
}
for (int i = data.length - 1; i > 0; i--) {
    data[i] = data[i] - data[i - 1];
}
```
What are the contents of `data` after both loops have executed?

A) `{3, 2, -3, 6, -2}`
B) `{6, 10, 4, 16, 12}`
C) `{6, 4, -6, 12, -4}`
D) `{6, -4, 6, -12, 4}`
E) `{6, 4, 0, 16, -4}`

::: {custom-style="QuestionBody"}
**Question 6.** Consider the following method. Assuming `nums` is a non‑null `int[]`, what integer value does the method return?
:::

```java
public static int computePositiveSum(int[] nums) {
    int sum = 0;
    for (int i = 0; i < nums.length; i++) {
        if (nums[i] > 0) {
            sum += nums[i];
        }
    }
    return sum;
}
```

For the call `computePositiveSum(new int[]{3, -2, 7, 0, 5})`, select the correct result.

A) `12`
B) `13`
C) `15`
D) `18`
E) `10`

::: {custom-style="QuestionBody"}
**Question 7.** Consider the following code segment.
:::

```java
ArrayList<Integer> list = new ArrayList<Integer>();
list.add(10);
list.add(20);
list.add(30);
list.remove(1);
list.add(0, 5);
list.set(2, 25);
list.remove(1);
```
After executing this code segment, what are the contents of **list**?

A) `[10, 25]`
B) `[5, 25, 30]`
C) `[5, 30]`
D) `[5, 25]`
E) `[5, 10, 25]`

::: {custom-style="QuestionBody"}
**Question 8.** Which of the following code fragments correctly declares an `int` array of length 6 and initializes each element so that `vals[i] = i * 2` for every index `i` from 0 through 5? Assume the fragment appears inside a method where `vals` has not been declared previously.
:::

A)
    ```java
    int[] vals = new int[6];
    for (int i = 0; i <= vals.length; i++) {
        vals[i] = i * 2;
    }
    ```
B)
    ```java
    int[] vals = new int[6]{0, 2, 4, 6, 8, 10};
    ```
C)
    ```java
    int[] vals = new int[6];
    for (int v : vals) {
        v = v * 2;
    }
    ```
D)
    ```java
    int[] vals = new int[6];
    for (int i = 0; i < vals.length; i++) {
        vals[i] = i * 2;
    }
    ```

::: {custom-style="QuestionBody"}
**Question 9.** Consider the following code segment.
:::

```java
int[] data = {5, 2, 9, 1, 4, 8};
for (int i = 1; i < data.length; i++) {
    if (data[i] > data[i - 1]) {
        data[i] = data[i] + data[i - 1];
    } else {
        data[i] = data[i] - data[i - 1];
    }
}
```
After the loop finishes, what is the value of `data[4]`?

A) `-5`
B) `1`
C) `0`
D) `-1`
E) `4`

::: {custom-style="QuestionBody"}
**Question 10.** Which of the following statements, placed inside a method body, will both compile **and** correctly initialize an `int` array named `evens` with the exact values `2, 4, 6, 8`?
:::

A) `int[] evens = new int[]{2, 4, 6, 8, 10};`
B) `int[] evens = new int[2, 4, 6, 8];`
C) `int[] evens = new int[4]{2, 4, 6, 8};`
D) `int[] evens; evens = {2, 4, 6, 8};`
E) `int[] evens = {2, 4, 6, 8};`

::: {custom-style="QuestionBody"}
**Question 11.** Consider the following code segment executed inside `main`:
:::

```java
int[] scores = {5, 10, 15};
int[] extra = new int[5];
for (int i = 0; i < scores.length; i++) {
    extra[i] = scores[i] * 2;
}
extra[4] = scores[0] + scores[2];
```
After the code runs, what are the contents of `extra`?

A) `{0, 0, 0, 0, 0}`
B) `{10, 20, 30, 0, 25}`
C) `{5, 10, 15, 0, 20}`
D) `{10, 20, 30, 0, 0}`
E) `{10, 20, 30, 0, 20}`

::: {custom-style="QuestionBody"}
**Question 12.** Consider the following code segment.
:::

```java
int[] nums = {9, 3, 7, 6, 15, 12, 4};
int count = 0;
for (int i = 0; i < nums.length; i++) {
    if (i % 2 == 0 && nums[i] % 3 == 0) {
        count++;
    }
}
```
After the loop finishes, what is the value of `count`?

A) `3`
B) `4`
C) `5`
D) `7`
E) `2`

::: {custom-style="QuestionBody"}
**Question 13.** Given the following initialization:
:::

```java
ArrayList<Integer> nums = new ArrayList<Integer>();
nums.add(1);
nums.add(2);
nums.add(3);
nums.add(4);
```
What is the exact effect of executing `nums.remove(2);`?

A) The list remains unchanged because `remove` expects an `Object`, not an `int`.
B) The code does not compile because the call to `remove` is ambiguous.
C) The element with value 2 is removed, leaving the list `[1, 3, 4]`.
D) The method returns the removed element, which is 2.
E) The element at index 2 is removed, leaving the list `[1, 2, 4]`.

::: {custom-style="QuestionBody"}
**Question 14.** Consider the following code segment.
:::

```java
import java.util.*;

public class Demo {
    public static void foo(ArrayList<String> list, int i, int j) {
        String temp = list.get(i);
        list.set(i, list.get(j));
        list.set(j, temp);
    }

    public static void main(String[] args) {
        ArrayList<String> colors = new ArrayList<>();
        colors.add("red");
        colors.add("green");
        colors.add("blue");
        colors.add("yellow");
        foo(colors, 1, 3);
        System.out.println(colors);
    }
}
```
What is printed to the console when this program is executed?

A) `[red, yellow, blue, green]`
B) `[red, yellow, blue, yellow]`
C) `[red, yellow, green, blue]`
D) `[blue, green, red, yellow]`
E) `[red, green, blue, yellow]`

::: {custom-style="QuestionBody"}
**Question 15.** The following code creates two `ArrayList<String>` objects. `src` initially contains `["apple", "banana", "apple", "cherry"]` and `dest` is empty. Which code fragment will, after execution, leave **both** lists containing only the elements that are not equal to `"apple"` (i.e., `dest` = `["banana","cherry"]` and `src` = `["banana","cherry"]`)?
:::

A)
    ```java
    for (String s : src) {
        if (s.equals("apple")) {
            src.remove(s);
        } else {
            dest.add(s);
        }
    }
    ```
B)
    ```java
    for (int i = 0; i < src.size(); i++) {
        if (src.get(i).equals("apple")) {
            src.remove(i);
            i--; 
        } else {
            dest.add(src.get(i));
        }
    }
    ```
C)
    ```java
    int i = 0;
    while (i < src.size()) {
        if (src.get(i).equals("apple")) {
            src.remove(i);
            i++;
        } else {
            dest.add(src.get(i));
            i++;
        }
    }
    ```
D)
    ```java
    for (int i = 0; i < src.size(); i++) {
        if (src.get(i).equals("apple")) {
            src.remove(i);
        } else {
            dest.add(src.get(i));
        }
    }
    ```
E)
    ```java
    for (int i = src.size() - 1; i >= 0; i--) {
        if (src.get(i).equals("apple")) {
            src.remove(i);
        } else {
            dest.add(src.get(i));
        }
    }
    ```

::: {custom-style="QuestionBody"}
**Question 16.** The following code is intended to fill `prefix` with the cumulative sums of the elements in `vals` (each entry holds the sum of all elements up to that index).  Only one change to the loop body will make it work correctly.  Which change should be made?
:::

```java
int[] vals   = {5, 3, 8, 2};
int[] prefix = new int[vals.length];
prefix[0] = vals[0];               
for (int i = 1; i < vals.length; i++) {
    prefix[i] = vals[i] + prefix[i]; // replace THIS line
}

// prefix *should* now equal {5, 8, 16, 18}
```
Choose the best option.

A) `prefix[i] = prefix[i-1] + vals[i-1];`
B) `prefix[i] = vals[i] + prefix[i-1];`
C) `prefix[i] = vals[i] + vals[i-1];`
D) `vals[i] = vals[i] + prefix[i];`
E) `prefix[i] = vals[i-1] + prefix[i];`

::: {custom-style="QuestionBody"}
**Question 17.** Examine the following Java code segment.
:::

```java
int[] values = {3, 7, 9, 12, 15};
values[1] = values[4];  
values[4] = values[1];
```
What is the contents of the array `values` after the code executes?

A) `{12, 7, 9, 3, 15}`
B) `{3, 15, 9, 12, 15}`
C) `{3, 15, 9, 7, 12}`
D) `{3, 7, 9, 12, 15}`
E) `{3, 15, 9, 12, 7}`

::: {custom-style="QuestionBody"}
**Question 18.** Consider the following code segment.
:::

```java
ArrayList<Integer> list = new ArrayList<>();
list.add(4);
list.add(7);
list.add(2);
list.add(9);
int total = 0;
for (int i = 0; i < list.size(); i++) {
    if (list.get(i) % 2 == 0) {
        total += list.get(i) * 2;
    } else {
        total += list.get(i) + 3;
    }
}
```
What is the final value of `total` after the loop finishes?

A) `26`
B) `22`
C) `18`
D) `44`
E) `34`

```{=openxml}
<w:p><w:r><w:br w:type="page"/></w:r></w:p>
```

## Quiz Key -- Version 1

1. C
2. C
3. B
4. C
5. C
6. C
7. D
8. D
9. D
10. E
11. E
12. E
13. E
14. A
15. B
16. B
17. B
18. E


```{=openxml}
<w:p><w:r><w:br w:type="page"/></w:r></w:p>
```

# Arrays and ArrayList Test 1 - Version 2

::: {custom-style="QuestionBody"}
**Question 1.** Consider the following code segment.
:::

```java
ArrayList<Integer> list = new ArrayList<Integer>();
list.add(10);
list.add(20);
list.add(30);
list.remove(1);
list.add(0, 5);
list.set(2, 25);
list.remove(1);
```
After executing this code segment, what are the contents of **list**?

A) `[5, 30]`
B) `[5, 25, 30]`
C) `[5, 25]`
D) `[5, 10, 25]`
E) `[10, 25]`

::: {custom-style="QuestionBody"}
**Question 2.** The following code creates two `ArrayList<String>` objects. `src` initially contains `["apple", "banana", "apple", "cherry"]` and `dest` is empty. Which code fragment will, after execution, leave **both** lists containing only the elements that are not equal to `"apple"` (i.e., `dest` = `["banana","cherry"]` and `src` = `["banana","cherry"]`)?
:::

A)
    ```java
    for (int i = src.size() - 1; i >= 0; i--) {
        if (src.get(i).equals("apple")) {
            src.remove(i);
        } else {
            dest.add(src.get(i));
        }
    }
    ```
B)
    ```java
    for (int i = 0; i < src.size(); i++) {
        if (src.get(i).equals("apple")) {
            src.remove(i);
            i--; 
        } else {
            dest.add(src.get(i));
        }
    }
    ```
C)
    ```java
    int i = 0;
    while (i < src.size()) {
        if (src.get(i).equals("apple")) {
            src.remove(i);
            i++;
        } else {
            dest.add(src.get(i));
            i++;
        }
    }
    ```
D)
    ```java
    for (int i = 0; i < src.size(); i++) {
        if (src.get(i).equals("apple")) {
            src.remove(i);
        } else {
            dest.add(src.get(i));
        }
    }
    ```
E)
    ```java
    for (String s : src) {
        if (s.equals("apple")) {
            src.remove(s);
        } else {
            dest.add(s);
        }
    }
    ```

::: {custom-style="QuestionBody"}
**Question 3.** Examine the following Java code segment.
:::

```java
int[] values = {3, 7, 9, 12, 15};
values[1] = values[4];  
values[4] = values[1];
```
What is the contents of the array `values` after the code executes?

A) `{3, 15, 9, 12, 15}`
B) `{3, 15, 9, 12, 7}`
C) `{3, 7, 9, 12, 15}`
D) `{12, 7, 9, 3, 15}`
E) `{3, 15, 9, 7, 12}`

::: {custom-style="QuestionBody"}
**Question 4.** The method below is intended to return the **last** index at which the given `target` string appears in `list`. If the target does not appear, the method should return `-1`.
:::

```java
public static int lastIndexOfTarget(ArrayList<String> list, String target) {
    for (int i = 0; i < list.size(); i++) {
        if (list.get(i).equals(target)) {
            return i; // supposed to find the last occurrence
        }
    }
    return -1;
}
```
Which **single** change will make the method work as described?

(Assume the rest of the program is correct.)

A) Change the loop condition to `i <= list.size() - 1`.
B) Change the loop header to `for (int i = list.size() - 1; i >= 0; i--)`.
C) Negate the comparison: `if (!list.get(i).equals(target)) return i;`.
D) Change the increment to `i--` but keep the initialization `int i = 0`.

::: {custom-style="QuestionBody"}
**Question 5.** Consider the following code segment.
:::

```java
ArrayList<Integer> list = new ArrayList<>();
list.add(4);
list.add(7);
list.add(2);
list.add(9);
int total = 0;
for (int i = 0; i < list.size(); i++) {
    if (list.get(i) % 2 == 0) {
        total += list.get(i) * 2;
    } else {
        total += list.get(i) + 3;
    }
}
```
What is the final value of `total` after the loop finishes?

A) `44`
B) `18`
C) `26`
D) `34`
E) `22`

::: {custom-style="QuestionBody"}
**Question 6.** Consider the following code segment executed inside `main`:
:::

```java
int[] scores = {5, 10, 15};
int[] extra = new int[5];
for (int i = 0; i < scores.length; i++) {
    extra[i] = scores[i] * 2;
}
extra[4] = scores[0] + scores[2];
```
After the code runs, what are the contents of `extra`?

A) `{10, 20, 30, 0, 25}`
B) `{0, 0, 0, 0, 0}`
C) `{10, 20, 30, 0, 0}`
D) `{5, 10, 15, 0, 20}`
E) `{10, 20, 30, 0, 20}`

::: {custom-style="QuestionBody"}
**Question 7.** Consider the following code segment.
:::

```java
int[] data = {3, 5, 2, 8, 6};
for (int i = 0; i < data.length; i++) {
    data[i] = data[i] * 2;
}
for (int i = data.length - 1; i > 0; i--) {
    data[i] = data[i] - data[i - 1];
}
```
What are the contents of `data` after both loops have executed?

A) `{6, 4, -6, 12, -4}`
B) `{6, -4, 6, -12, 4}`
C) `{6, 10, 4, 16, 12}`
D) `{3, 2, -3, 6, -2}`
E) `{6, 4, 0, 16, -4}`

::: {custom-style="QuestionBody"}
**Question 8.** The code below creates an `ArrayList<String>` named `words`.
:::

```java
ArrayList<String> words = new ArrayList<>();
words.add("alpha");
words.add("beta");
words.add("gamma");
words.add("delta");
```
Which of the following code fragments, placed after the statements above, will print the elements of `words` in reverse order (one per line) without using `Collections.reverse` or creating a second list?

Select the **single** fragment that works correctly for any list size.

A)
    ```java
    for (int i = words.size(); i > 0; i--) {
        System.out.println(words.get(i));
    }
    ```
B)
    ```java
    int i = words.size();
    while (i-- > 1) {
        System.out.println(words.get(i));
    }
    ```
C)
    ```java
    for (String s : words) {
        System.out.println(s);
    }
    ```
D)
    ```java
    for (int i = words.size() - 1; i >= 0; i--) {
        System.out.println(words.get(i));
    }
    ```
E)
    ```java
    for (int i = 0; i < words.size(); i++) {
        System.out.println(words.get(i));
    }
    ```

::: {custom-style="QuestionBody"}
**Question 9.** Consider the following code segment.
:::

```java
import java.util.*;

public class Demo {
    public static void foo(ArrayList<String> list, int i, int j) {
        String temp = list.get(i);
        list.set(i, list.get(j));
        list.set(j, temp);
    }

    public static void main(String[] args) {
        ArrayList<String> colors = new ArrayList<>();
        colors.add("red");
        colors.add("green");
        colors.add("blue");
        colors.add("yellow");
        foo(colors, 1, 3);
        System.out.println(colors);
    }
}
```
What is printed to the console when this program is executed?

A) `[red, green, blue, yellow]`
B) `[red, yellow, blue, yellow]`
C) `[red, yellow, blue, green]`
D) `[red, yellow, green, blue]`
E) `[blue, green, red, yellow]`

::: {custom-style="QuestionBody"}
**Question 10.** Which of the following statements, placed inside a method body, will both compile **and** correctly initialize an `int` array named `evens` with the exact values `2, 4, 6, 8`?
:::

A) `int[] evens = new int[2, 4, 6, 8];`
B) `int[] evens; evens = {2, 4, 6, 8};`
C) `int[] evens = new int[]{2, 4, 6, 8, 10};`
D) `int[] evens = new int[4]{2, 4, 6, 8};`
E) `int[] evens = {2, 4, 6, 8};`

::: {custom-style="QuestionBody"}
**Question 11.** Given the following initialization:
:::

```java
ArrayList<Integer> nums = new ArrayList<Integer>();
nums.add(1);
nums.add(2);
nums.add(3);
nums.add(4);
```
What is the exact effect of executing `nums.remove(2);`?

A) The element with value 2 is removed, leaving the list `[1, 3, 4]`.
B) The list remains unchanged because `remove` expects an `Object`, not an `int`.
C) The element at index 2 is removed, leaving the list `[1, 2, 4]`.
D) The code does not compile because the call to `remove` is ambiguous.
E) The method returns the removed element, which is 2.

::: {custom-style="QuestionBody"}
**Question 12.** The method below is intended to return the average of the values in `values` as a `double`. However, it usually returns an incorrect value. Which of the following can fix the problem?
:::

```java
public static double average(int[] values) {
    int total = 0;
    for (int v : values) {
        total += v;
    }
    return total / values.length;
}
```
Which single change will make the method return the correct average as a `double`?
Select the best modification.

A) Change the return statement to `return total / (values.length - 1);`.
B) Change the return statement to `return (double) total / values.length;`.
C) Change the method’s return type from `double` to `int`.
D) Change the loop header to `for (int i = 0; i <= values.length; i++) total += values[i];`.

::: {custom-style="QuestionBody"}
**Question 13.** The following code is intended to fill `prefix` with the cumulative sums of the elements in `vals` (each entry holds the sum of all elements up to that index).  Only one change to the loop body will make it work correctly.  Which change should be made?
:::

```java
int[] vals   = {5, 3, 8, 2};
int[] prefix = new int[vals.length];
prefix[0] = vals[0];               
for (int i = 1; i < vals.length; i++) {
    prefix[i] = vals[i] + prefix[i]; // replace THIS line
}

// prefix *should* now equal {5, 8, 16, 18}
```
Choose the best option.

A) `vals[i] = vals[i] + prefix[i];`
B) `prefix[i] = vals[i-1] + prefix[i];`
C) `prefix[i] = vals[i] + vals[i-1];`
D) `prefix[i] = vals[i] + prefix[i-1];`
E) `prefix[i] = prefix[i-1] + vals[i-1];`

::: {custom-style="QuestionBody"}
**Question 14.** Consider the following method. Assuming `nums` is a non‑null `int[]`, what integer value does the method return?
:::

```java
public static int computePositiveSum(int[] nums) {
    int sum = 0;
    for (int i = 0; i < nums.length; i++) {
        if (nums[i] > 0) {
            sum += nums[i];
        }
    }
    return sum;
}
```

For the call `computePositiveSum(new int[]{3, -2, 7, 0, 5})`, select the correct result.

A) `13`
B) `10`
C) `18`
D) `15`
E) `12`

::: {custom-style="QuestionBody"}
**Question 15.** The following code creates an `ArrayList<Integer>` named `numbers`:
:::

```java
ArrayList<Integer> numbers = new ArrayList<>();
numbers.add(1);
numbers.add(2);
numbers.add(4);
numbers.add(5);
numbers.add(6);
numbers.add(7);
numbers.add(8);
numbers.add(9);
numbers.add(10);
```
One of the code fragments below is inserted after these statements. After the fragment executes, `numbers` must contain **only the odd integers that were originally in the list** (i.e., `[1, 5, 7, 9]`) and the program must **not throw any exception**. Which fragment satisfies the requirement?

**A)**
```java
for (int n : numbers) {
    if (n % 2 == 0) {
        numbers.remove(Integer.valueOf(n));
    }
}
```

**B)**
```java
for (int i = 0; i < numbers.size(); i++) {
    if (numbers.get(i) % 2 == 0) {
        numbers.remove(i);
    }
}
```

**C)**
```java
for (int i = numbers.size(); i >= 0; i--) {
    if (numbers.get(i) % 2 == 0) {
        numbers.remove(i);
    }
}
```

**D)**
```java
for (int i = numbers.size() - 1; i >= 0; i--) {
    if (numbers.get(i) % 2 == 0) {
        numbers.remove(i);
    }
}
```

A) B
B) A
C) D
D) C

::: {custom-style="QuestionBody"}
**Question 16.** Consider the following code segment.
:::

```java
int[] nums = {9, 3, 7, 6, 15, 12, 4};
int count = 0;
for (int i = 0; i < nums.length; i++) {
    if (i % 2 == 0 && nums[i] % 3 == 0) {
        count++;
    }
}
```
After the loop finishes, what is the value of `count`?

A) `3`
B) `7`
C) `5`
D) `2`
E) `4`

::: {custom-style="QuestionBody"}
**Question 17.** Consider the following code segment.
:::

```java
int[] data = {5, 2, 9, 1, 4, 8};
for (int i = 1; i < data.length; i++) {
    if (data[i] > data[i - 1]) {
        data[i] = data[i] + data[i - 1];
    } else {
        data[i] = data[i] - data[i - 1];
    }
}
```
After the loop finishes, what is the value of `data[4]`?

A) `4`
B) `-5`
C) `0`
D) `-1`
E) `1`

::: {custom-style="QuestionBody"}
**Question 18.** Which of the following code fragments correctly declares an `int` array of length 6 and initializes each element so that `vals[i] = i * 2` for every index `i` from 0 through 5? Assume the fragment appears inside a method where `vals` has not been declared previously.
:::

A)
    ```java
    int[] vals = new int[6];
    for (int i = 0; i < vals.length; i++) {
        vals[i] = i * 2;
    }
    ```
B)
    ```java
    int[] vals = new int[6];
    for (int v : vals) {
        v = v * 2;
    }
    ```
C)
    ```java
    int[] vals = new int[6];
    for (int i = 0; i <= vals.length; i++) {
        vals[i] = i * 2;
    }
    ```
D)
    ```java
    int[] vals = new int[6]{0, 2, 4, 6, 8, 10};
    ```

```{=openxml}
<w:p><w:r><w:br w:type="page"/></w:r></w:p>
```

## Quiz Key -- Version 2

1. C
2. B
3. A
4. B
5. D
6. E
7. A
8. D
9. C
10. E
11. C
12. B
13. D
14. D
15. C
16. D
17. D
18. A
