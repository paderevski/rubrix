# AP CS Questions

1. Consider the following code segment.

```java
int a = 5;
int b = 3;
int c = a;
a = a + b;
b = c * a;
System.out.println(a + " " + b + " " + c);
```

What is printed as a result of executing this code segment?

    A) `8 64 5`
    B) `8 5 40`
    C) `8 40 5`
    D) `8 64 8`

2. In the following statement, `a` and `b` are properly declared and initialized `int` variables.

```java
boolean flag = ((a % 2 == 0) && (b % 2 == 0)) || 
                ((a % 2 != 0) && (b % 2 != 0));
```

Which of the following best describes the conditions in which `flag` is assigned the value `true`?

    A) When one of `a` and `b` is even and the other is odd
    B) When `a` and `b` are equal (`a == b`)
    C) When `a` and `b` are both even or both odd
    D) True for all integer values of `a` and `b`

3. Consider the following `Gadget` class.

```java
public class Gadget
{
    private String model;
    private int year;
    private double price;

    public Gadget(String m, int y)
    {
        model = m;
        year = y;
        price = 0.0;
    }
    // There are no other constructors.
}
```

Which of the following code segments, appearing in a class other than `Gadget`, will correctly create an instance of a `Gadget` object?

    A) ```java
    Gadget g = new Gadget("iPhone", 2020);
    ```
    B) ```java
    Gadget g = Gadget("iPhone", 2020);
    ```
    C) ```java
    Gadget g = new Gadget("iPhone", "2020");
    ```
    D) ```java
    Gadget g = new Gadget(2020, "iPhone");
    ```

4. Consider the following code segment.

```java
int a = (7 + 3) * 2 - 5 / 2;
int b = a % 4 + a / 5;
```

What is the value of `b` after this code segment is executed?

    A) `4`
    B) `5`
    C) `6`
    D) `3`

5. Consider the following code segment.

```java
double x = 80.0;
double y = 0.0;

if (x >= 100.0)
{
    y = 30.0;
}
else if (x > 80.0)
{
    y = 20.0;
}
else if (x >= 60.0)
{
    y = 15.0;
}
else
{
    y = 5.0;
}
System.out.println(x - y);
```

What is printed as a result of executing this code segment?

    A) `65.0`
    B) `80.0`
    C) `75.0`
    D) `60.0`

6. Consider the following code segment.

```java
for (int j = 6; j >= 0; j -= 2) // Line 1
{
    System.out.print(j);
}
```

This code segment is intended to produce only the following output. 

`642`

It does not work as intended.

Specify a single change (give the line number and the exact change) that can be made so that this code segment works as intended.

    A) In line 1, changing the initialization `j = 6` to `j = 5`
    B) In line 1, changing the condition `j >= 0` to `j > 0`
    C) In line 1, changing the condition `j >= 0` to `j > 2`
    D) In line 1, changing the decrement `j -= 2` to `j -= 1`

7. Consider the following code segment.

```java
String s = "RSTUVW";
String x = /* missing code */;
System.out.println(x);
```

Which of the following expressions can be used to replace `/* missing code */` so that this code segment prints the string "WRST"?

    A) `s.substring(0, 3) + s.substring(5)`
    B) `s.substring(4) + s.substring(0, 3)`
    C) `s.charAt(5) + s.charAt(0) + s.charAt(1) + s.charAt(2)`
    D) `s.substring(5) + s.substring(0, 3)`

8. Consider the following code segment.

```java
double a = 3.0;
double b = 8.0;
double c = a / (int)b;
double d = (int)(a / b);
System.out.println(c + " " + d);
```

What is printed as a result of executing this code segment?

    A) `0.375, 0.0`
    B) `0.0, 0.0`
    C) `0.375, 1.0`
    D) `0.375, 0.375`

9. Consider the following class definitions.

```java
public class Box
{
    private int x;
    private int y;

    public Box(int xVal, int yVal)
    {
        x = xVal;
        y = yVal;
    }

    private int foo()
    {
        return x * y;
    }
}

public class Checker
{
    public static boolean compare(Box b1, Box b2)
    {
        if (b1.foo() > b2.foo())
        {
            return true;
        }
        return false;
    }
}
```

Which of the following best explains why an error occurs when the classes are compiled?

    A) The `foo` method cannot be referenced from the static method `compare`.
    B) The `foo` method cannot be accessed from a class other than `Box`.
    C) The `foo` method must be declared `static` because `compare` is static.
    D) The `compare` method must be an instance method (not `static`) to call `foo`.

10. Assume the following class is defined in its own file.

```java
public class Quad {
    private int p;
    private int q;

    public Quad(int p, int q) {
        this.p = p;
        this.q = q;
    }

    public int sum() {
        return p + q;
    }

    public int sum(int extra) {
        return p + q + extra;
    }

    private int secret() {
        return p * q;
    }

    private static int staticSum(Quad obj) {
        return obj.p + obj.q;
    }
}
```

In another class you have created an instance:

```java
Quad myQuad = new Quad(2, 3);
```

Which of the following statements **will compile without error**?

    A) `int a = myQuad.sum();`
    B) `int b = myQuad.secret();`
    C) `int c = Quad.sum(5);`
    D) `int e = myQuad.sum(2, 3);`
    E) `int d = Quad.staticSum(myQuad);`

11. Consider the following class:

```java
public class Container {
    private int z;

    public Container(int initial) {
        z = initial;
    }

    public int getZ() {
        return z;
    }
}
```

Write a method named `foo` that could be added to the `Container` class so that other classes can modify the value of `z`.

    A) ```java
    public void foo(int num)
    {
    z == num;
    }
    ```
    B) ```java
    public void foo(int num)
    {
    z = num;
    }
    ```
    C) ```java
    public void foo(int z)
    {
    z = z;
    }
    ```
    D) ```java
    private void foo(int num)
    {
    z = num;
    }
    ```

12. Consider the following code segment.

```java
for (int i = 1; i <= 6; i++)
{
    for (int j = 0; j < i; j++)
    {
        System.out.println("*");
        // Line 4
    }
}
```

How many times will the statement in line 4 be executed as a result of running this code segment?

    A) `27`
    B) `36`
    C) `15`
    D) `21`

13. Assume that `isMember`, `isExpired`, and `hasBadge` are boolean variables that have been properly declared and initialized. Which boolean expression is logically equivalent to the expression

```java
!(isMember || isExpired) && hasBadge
```

Write an equivalent expression.

    A) `(!isMember || !isExpired) && hasBadge`
    B) `!(isMember && isExpired) && hasBadge`
    C) `(!isMember && !isExpired) && hasBadge`
    D) `(!isMember && isExpired) && hasBadge`

14. Consider the following code segment.

```java
String str = "mississippi";
String target = "s";
int j = str.indexOf(target);

while (j >= 0)
{
    System.out.print(str.substring(0, j + 1) + " ");
    str = str.substring(j + 1);
    j = str.indexOf(target);
}
```

What is printed as a result of executing this code segment?

    A) `mis mis mis`
    B) `mis sis sip`
    C) `mis s s s`
    D) `mis s is s`

15. The following method is intended to return the minimum value that appears in the `int` array `arr`.

```java
public static int foo(int[] arr)
{
    int m = 0;
    for (int x : arr)
    {
        if (x < m)
        {
            m = x;
        }
    }
    return m;
}
```

The method works as intended for some, but not all, `int` arrays. Which of the following best describes the conditions in which the method does not return the intended result?

    A) When the minimum value in `arr` is positive (greater than 0)
    B) When the minimum value in `arr` is negative (less than 0)
    C) When the minimum value in `arr` is the first element (`arr[0]`)
    D) When the maximum value in `arr` is positive (greater than 0)

16. Consider the following statement.

```java
int x = (int) (Math.random() * 8) - 3;
```

Describe all possible integer values that `x` can assume after this statement executes. In your answer, state the smallest and largest possible values and explain whether each endpoint is inclusive or exclusive based on how `Math.random()` and the cast to `int` behave.

    A) It generates a random integer between -2 and 5, inclusive, and assigns it to `x`.
    B) It generates a random integer between -3 and 4, inclusive, and assigns it to `x`.
    C) It generates a random integer between -3 and 5, inclusive, and assigns it to `x`.
    D) It always assigns the integer -3 to `x`.

17. Consider the following code segment.

```java
int x = 729;
while (x % 9 == 0)
{
    x /= 3;
}
System.out.println(x);
```

What is printed as a result of executing this code segment?

    A) `1`
    B) `729`
    C) `9`
    D) `3`

18. Consider the following code segment.

```java
for (int j = 4; j > 0; j--)
{
    for (/* missing code */)
    {
        System.out.print(j + " ");
    }
    System.out.println();
}
```

This code segment is intended to produce the following output.

```
4 4 4 4
3 3 3
2 2
1
```

Which of the following can be used to replace `/* missing code */` so that this code segment works as intended?

    A) ```java
    int k = 0; k <= j; k++
    ```
    B) ```java
    int k = 0; k < j; k--
    ```
    C) ```java
    int k = 0; k < j; k++
    ```
    D) ```java
    int k = 0; k == j; k++
    ```

19. The following class definition does not compile.

```java
public class Gadget
{
    private String a;
    private String b;
    private int c;

    public Gadget(String s1, String s2)
    {
        a = s1;
        b = s2;
        c = 0;
    }

    public void increment()
    {
        c++;
    }

    public String doSomething()
    {
        return s1 + ", " + s2;
    }
}
```

Which of the following best explains why the class will not compile?

    A) The `doSomething` method is declared `static`, so it cannot access the instance variables `a` and `b`.
    B) Assigning `a = s1` and `b = s2` in the constructor makes `s1` and `s2` become aliases for the fields, so `doSomething` can use `s1` and `s2`.
    C) The `doSomething` method does not have access to variables named `s1` or `s2`.
    D) The class has three instance variables (`a`, `b`, `c`), but its constructor has only two parameters (`s1`, `s2`).

20. Consider the following method.

```java
public static void mystery(int[] arr, int idx)
{
    arr[idx + 1] = idx;
}
```

The following code segment appears in a method in the same class as `mystery`.

```java
int[] data = new int[5];
mystery(data, 1);
mystery(data, 3);
```

What are the contents of `data` after executing this code segment?

    A) `{0, 0, 2, 0, 4}`
    B) `{0, 1, 0, 3, 0}`
    C) `{0, 0, 0, 0, 0}`
    D) `{0, 0, 1, 0, 3}`

21. Consider the following code segment.

```java
ArrayList<Integer> list = new ArrayList<Integer>();
list.add(10);
list.add(20);
list.add(1, 30);
list.add(0, 40);
list.set(2, 50);
list.remove(3);
```

What are the contents of `list` after executing this code segment?

    A) `[40, 10, 50]`
    B) `[40, 50, 20]`
    C) `[40, 30, 50]`
    D) `[40, 10, 50, 20]`

22. Consider the following class definition.

```java
public class Gizmo
{
    private static int n = 0;
    private int watt;
    private String color;

    public Gizmo()
    {
        watt = 10;
        color = "white";
    }

    public Gizmo(int w, String c)
    {
        watt = w;
        color = c;
        n++;
    }

    public static int mystery()
    {
        return n;
    }
}
```

The following code segment appears in a class other than `Gizmo`.

```java
Gizmo g1 = new Gizmo(12, "red");
Gizmo g2 = new Gizmo();
Gizmo g3 = new Gizmo(8, "blue");
Gizmo g4 = new Gizmo(15, "green");
System.out.println(Gizmo.mystery());
```

What is printed as a result of executing this code segment if it is executed before any other instances of `Gizmo` are constructed?

    A) 2
    B) 1
    C) 3
    D) 4

23. A researcher wants to investigate whether students who took an engineering elective were more likely to participate in the school's robotics team. The researcher has access to the following data sets.

- Data set 1 contains an entry for each club at the school. Each entry includes the club name and a list of the names of the club members.
- Data set 2 contains an entry for each member of the robotics team. Each entry contains the member's name, the number of competitions the member has attended, and the member's favorite elective.
- Data set 3 contains an entry for each student at the school. Each entry contains the student's name, a list of the courses the student took, and a list of the clubs the student participated in.
- Data set 4 contains an entry for each student at the school. Each entry contains the student's name, the student's favorite elective, and the student's favorite club.

Which data set is most appropriate for the researcher's investigation? Explain your choice and briefly state why the other data sets are less suitable.

    A) Data set 3
    B) Data set 4
    C) Data set 2
    D) Data set 1

24. Consider the following method.

```java
public static void mystery(int num)
{
    if (num > -50)
    {
        mystery(num * -2 + 1);
        System.out.print(num + " ");
    }
}
```

What is printed as a result of the call `mystery(7)`?

    A) `7 -13 27`
    B) `28 -12 8`
    C) `27 -13 7`
    D) `29 -15 7`

25. The following incomplete method is intended to return the median (middle) value among its three parameters (i.e., the value that is neither the maximum nor the minimum).

```java
public static int mystery(int a, int b, int c)
{
    /* missing code */
}
```

Which of the following can be used to replace `/* missing code */` so that the method works as intended?

    A) ```java
    if ((a <= b || a >= c) || (a >= b || a <= c))
    return a;
    else if ((b <= a || b >= c) || (b >= a || b <= c))
    return b;
    else
    return c;
    ```
    B) ```java
    if (a <= b && b <= c)
    return b;
    else if (b <= a && a <= c)
    return a;
    else
    return c;
    ```
    C) ```java
    if ((a <= b && a >= c) || (a >= b && a <= c))
    return a;
    else if ((b <= a && b >= c) || (b >= a && b <= c))
    return b;
    else
    return c;
    ```
    D) ```java
    if ((a < b && a > c) || (a > b && a < c))
    return a;
    else if ((b < a && b > c) || (b > a && b < c))
    return b;
    else
    return c;
    ```

26. Consider the following code segment.

```java
int[][] mat = {{1, 2, 3},
               {4, 5, 6},
               {7, 8, 9}};
ArrayList<Integer> list = new ArrayList<Integer>();

/* missing code */
```

After executing this code segment, `list` should contain `[7, 4, 1, 8, 5, 2, 9, 6, 3]`. Which of the following can replace `/* missing code */` to produce this result?

    A) ```java
    for (int k = 0; k < mat[0].length; k++)
    {
    for (int j = mat.length - 1; j >= 0; j--)
    {
    list.add(mat[k][j]);
    }
    }
    ```
    B) ```java
    for (int k = 0; k < mat[0].length; k++)
    {
    for (int j = 0; j < mat.length; j++)
    {
    list.add(mat[j][k]);
    }
    }
    ```
    C) ```java
    for (int k = 0; k < mat[0].length; k++)
    {
    for (int j = 0; j < mat.length; j++)
    {
    list.add(0, mat[j][k]);
    }
    }
    ```
    D) ```java
    for (int k = 0; k < mat[0].length; k++)
    {
    for (int j = mat.length - 1; j >= 0; j--)
    {
    list.add(mat[j][k]);
    }
    }
    ```

27. Consider the following code segment.

```java
int[] arr = {3, 6, 9, 12, 15, 18, 21};

for (int i = 0; i < arr.length; i += 2)
{
    arr[i] += i;
}
```

What are the contents of `arr` after executing this code segment?

    A) `{3, 6, 11, 12, 19, 18, 27}`
    B) `{3, 6, 11, 12, 19, 18, 21}`
    C) `{3, 7, 11, 15, 19, 23, 27}`
    D) `{4, 6, 10, 12, 16, 18, 22}`

28. Consider the following code segment.

```java
ArrayList<String> listA = new ArrayList<String>();
listA.add("p");
listA.add("q");
listA.add("r");
listA.add("q");
ArrayList<String> listB = new ArrayList<String>();
/* missing code */
```

After executing this code segment, `listB` should contain `["q","r","q","p","p","q","r","q”]`. Which of the following can replace `/* missing code */` to produce this result?

    A) ```java
    for (int i = listA.size() - 1; i >= 0; i--)
    {
    listB.add(listA.get(i));
    }
    for (int i = 0; i < listA.size(); i++)
    {
    listB.add(0, listA.get(i));
    }
    ```
    B) ```java
    for (int i = listA.size() - 1; i >= 0; i--)
    {
    listB.add(listA.get(i));
    }
    for (int i = 0; i < listA.size(); i++)
    {
    listB.add(listA.get(i));
    }
    ```
    C) ```java
    for (int i = listA.size() - 1; i > 0; i--)
    {
    listB.add(listA.get(i));
    }
    for (int i = 0; i < listA.size(); i++)
    {
    listB.add(listA.get(i));
    }
    ```
    D) ```java
    for (int i = listA.size() - 1; i >= 0; i--)
    {
    listB.add(listA.get(i));
    }
    for (int i = 1; i < listA.size(); i++)
    {
    listB.add(listA.get(i));
    }
    ```

29. Consider the following method.

```java
public static boolean mystery(ArrayList<Integer> list)
{
    int prev = Integer.MIN_VALUE;
    for (int n : list)
    {
        if (n < prev)
        {
            return false;
        }
        prev = n;
    }
    return true;
}
```

Which of the following best describes the behavior of the method?

    A) It returns `true` if `list` is sorted from greatest to least and returns `false` otherwise.
    B) It returns `true` if `list` is sorted from least to greatest and returns `false` otherwise.
    C) It returns `true` if every element in `list` is greater than or equal to the minimum element in `list` and returns `false` otherwise.
    D) It returns `true` if `list` is strictly increasing (each element greater than the previous) and returns `false` otherwise.

30. Consider the following code segment.

```java
int[][] mystery = new int[4][3];

for (int r = 0; r < mystery.length; r++)
{
    for (int c = 0; c < mystery[0].length; c++)
    {
        if (r + c == mystery.length - 1)
        {
            mystery[r][c] = c + 1;
        }
    }
}
```

What are the contents of `mystery` after executing this code segment?  Write the contents as a 2D array literal (for example: `{{...}, {...}, ...}`).

    A) ```
    {{0, 0, 0},
    {0, 0, 2},
    {0, 1, 0},
    {0, 0, 0}}
    ```
    B) ```
    {{0, 0, 0},
    {0, 0, 3},
    {0, 2, 0},
    {0, 0, 0}}
    ```
    C) ```
    {{0, 0, 0},
    {0, 0, 3},
    {0, 2, 0},
    {1, 0, 0}}
    ```
    D) ```
    {{0, 0, 0},
    {0, 0, 0},
    {0, 0, 3},
    {0, 2, 0}}
    ```

31. Consider the following method. When an element of the two-dimensional array `arr` is accessed, the first index is used to specify the row and the second index is used to specify the column.

```java
public static int mystery(int[][] arr, int x, int target)
{
    int n = 0;
    for (int i = 0; i < arr.length; i++)
    {
        if (x < arr[i].length && arr[i][x] == target)
        {
            n++;
        }
    }
    return n;
}
```

Which of the following best describes the value returned by the method?

    A) The number of times that an element equal to `target` appears anywhere in `arr`
    B) The number of rows in `arr` that contain column `x`
    C) The number of times that an element equal to `target` appears in row `x` of `arr`
    D) The number of times that an element equal to `target` appears in column `x` of `arr`

32. The following method implements a recursive binary search variant.

```java
public static int mystery(int[] arr, int low, int high, int target)
{
    if (low <= high)
    {
        int mid = (low + high + 1) / 2;
        if (target < arr[mid])
        {
            return mystery(arr, low, mid - 1, target);
        }
        else if (target > arr[mid])
        {
            return mystery(arr, mid + 1, high, target);
        }
        else
        {
            return mid;
        }
    }
    return -1;
}
```

The following code segment appears in a method in the same class as `mystery`:

```java
int[] values = {4, 4, 4, 5, 6, 7, 8};
int idx = mystery(values, 0, values.length - 1, 4);
```

What is the value of `idx` after executing the code segment?

    A) `1`
    B) `3`
    C) `2`
    D) `0`

33. Consider the following method that implements insertion sort on an ArrayList of Integer objects.

```java
public static void doSomething(ArrayList<Integer> arr)
{
    for (int j = 1; j < arr.size(); j++)
    {
        int temp = arr.get(j);
        int k = j;
        while (k > 0 && temp < arr.get(k - 1))
        {
            arr.set(k, arr.get(k - 1));
            k--;
        }
        arr.set(k, temp);
        /* end of outer loop */
    }
}
```

Assume `doSomething` is called with an ArrayList initialized with the following Integer objects:

`[45, 15, 35, 25, 55, 5]`

What will the contents of `arr` be after three passes of the outer loop (i.e., when `j == 3` at the point indicated by `/* end of outer loop */`)?

    A) `[15, 25, 55, 55, 55, 5]`
    B) `[15, 35, 45, 25, 55, 5]`
    C) `[45, 35, 25, 15, 55, 5]`
    D) `[15, 25, 35, 45, 55, 5]`

34. Consider the following code segment.

```java
String[] words = {"clam", "moss", "sand", "dog", "goat", "tire"};
int m = -1;

for (int i = 0; i < words.length; i++)
{
    for (int j = i + 1; j < words.length; j++)
    {
        if (words[i].charAt(words[i].length() - 1) == words[j].charAt(0))
        {
            m = j;
        }
    }
}
```

What is the value of `m` after executing this code segment?

    A) 1
    B) 4
    C) 6
    D) 5

35. Consider the following code segment.

```java
int j = 7;
int s = 0;
while (j >= 2)
{
    j--;
    s += j;
}
```

Which of the following code segments assigns the same value to `s` as the preceding code segment?

    A) ```java
    int s = 0;
    for (int j = 1; j <= 7; j++)
    {
    s += j;
    }
    ```
    B) ```java
    int s = 0;
    for (int j = 0; j < 6; j++)
    {
    s += j;
    }
    ```
    C) ```java
    int s = 0;
    for (int j = 1; j <= 6; j++)
    {
    s += j;
    }
    ```
    D) ```java
    int s = 0;
    for (int j = 2; j <= 6; j++)
    {
    s += j;
    }
    ```

## Answers

1. C
2. C
3. A
4. B
5. A
6. B
7. D
8. A
9. B
10. A
11. B
12. D
13. C
14. D
15. A
16. B
17. D
18. C
19. C
20. D
21. A
22. C
23. A
24. C
25. C
26. D
27. A
28. B
29. B
30. C
31. D
32. A
33. D
34. D
35. C
