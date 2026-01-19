2. Consider the following code segment.

```java
int a = 5;
int b = 3;
int c = a;
a = a + b;
b = c * a;
System.out.println(a + " " + b + " " + c);
```

What is printed as a result of executing this code segment?

a. `8 40 5`
a. `8 64 5`
a. `8 64 8`
a. `8 5 40`

3. In the following statement, `a` and `b` are properly declared and initialized `int` variables.

```java
boolean flag = ((a % 2 == 0) && (b % 2 == 0)) || ((a % 2 != 0) && (b % 2 != 0));
```

Which of the following best describes the conditions in which `flag` is assigned the value `true`?

a. When `a` and `b` are both even or both odd
a. When one of `a` and `b` is even and the other is odd
a. True for all integer values of `a` and `b`
a. When `a` and `b` are equal (`a == b`)

4. Consider the following `Gadget` class.

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

a. ```java
Gadget g = new Gadget("iPhone", 2020);
```
a. ```java
Gadget g = new Gadget(2020, "iPhone");
```
a. ```java
Gadget g = new Gadget("iPhone", "2020");
```
a. ```java
Gadget g = Gadget("iPhone", 2020);
```

5. Consider the following code segment.

```java
int a = (7 + 3) * 2 - 5 / 2;
int b = a % 4 + a / 5;
```

What is the value of `b` after this code segment is executed?

a. `5`
a. `4`
a. `6`
a. `3`

6. Consider the following code segment.

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

a. `65.0`
a. `60.0`
a. `80.0`
a. `75.0`

7. Consider the following code segment.

```java
for (int j = 6; j >= 0; j -= 2) // Line 1
{
    System.out.print(j);
}
```

This code segment is intended to produce only the following output. It does not work as intended.
`642`

Specify a single change (give the line number and the exact change) that can be made so that this code segment works as intended.

a. In line 1, changing the condition `j >= 0` to `j > 0`
a. In line 1, changing the initialization `j = 6` to `j = 5`
a. In line 1, changing the condition `j >= 0` to `j > 2`
a. In line 1, changing the decrement `j -= 2` to `j -= 1`

8. Consider the following code segment.

```java
String s = "RSTUVW";
String x = /* missing code */;
System.out.println(x);
```

Which of the following expressions can be used to replace `/* missing code */` so that this code segment prints the string "WRST"?

a. `s.substring(5) + s.substring(0, 3)`
a. `s.substring(4) + s.substring(0, 3)`
a. `s.substring(0, 3) + s.substring(5)`
a. `s.charAt(5) + s.charAt(0) + s.charAt(1) + s.charAt(2)`

9. Consider the following code segment.

```java
double a = 3.0;
double b = 8.0;
double c = a / (int)b;
double d = (int)(a / b);
System.out.println(c + " " + d);
```

What is printed as a result of executing this code segment?

a. `0.375, 0.0`
a. `0.0, 0.0`
a. `0.375, 0.375`
a. `0.375, 1.0`

10. Consider the following class definitions.

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

a. The `foo` method cannot be accessed from a class other than `Box`.
a. The `foo` method cannot be referenced from the static method `compare`.
a. The `foo` method must be declared `static` because `compare` is static.
a. The `compare` method must be an instance method (not `static`) to call `foo`.

11. Consider the following class declaration.

```java
public class Trio
{
    private int a;
    private int b;

    public Trio(int a, int b1)
    {
        this.a = a;
        b = b1;
    }

    public int mystery()
    {
        return a;
    }

    public int mystery(int i)
    {
        return b + i;
    }
}
```

Assume that `myTrio` is a properly declared and initialized `Trio` object that appears in a class other than `Trio`. Which of the following code segments will compile without error?


a. ```java
int val = myTrio.mystery();
int val = myTrio.mystery(3);
```
a. ```java
int val = myTrio.a;
int val = myTrio.b;
```
a. ```java
int val = myTrio.a;
int val = myTrio.b1;
```
a. ```java
int val = myTrio.b;
int val = myTrio.b1;
```

1.  Consider the following class:

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

Write a method named `foo` that could be added to the `Container` class so that other classes can modify the value of `z`. The method should have the appropriate access modifier and should not return a value. Provide the exact method declaration and body as you would type it inside the class (include the braces).

a. ```java
public void foo(int num)
{
    z = num;
}
```
a. ```java
public void foo(int z)
{
    z = z;
}
```
a. ```java
private void foo(int num)
{
    z = num;
}
```
a. ```java
public void foo(int num)
{
    z == num;
}
```

13. Consider the following code segment.

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

a. `21`
a. `15`
a. `27`
a. `36`

14. Assume that `isMember`, `isExpired`, and `hasBadge` are boolean variables that have been properly declared and initialized. Which boolean expression is logically equivalent to the expression

```java
!(isMember || isExpired) && hasBadge
```

Write an equivalent expression using only the variables `isMember`, `isExpired`, `hasBadge` and the operators `&&`, `||`, and `!`.

a. `(!isMember && !isExpired) && hasBadge`
a. `(!isMember || !isExpired) && hasBadge`
a. `(!isMember && isExpired) && hasBadge`
a. `!(isMember && isExpired) && hasBadge`

15. Consider the following code segment.

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

a. `mis s is s`
a. `mis s s s`
a. `mis sis sip`
a. `mis mis mis`

16. The following method is intended to return the minimum value that appears in the `int` array `arr`.

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

a. When the minimum value in `arr` is positive (greater than 0)
a. When the minimum value in `arr` is negative (less than 0)
a. When the maximum value in `arr` is positive (greater than 0)
a. When the minimum value in `arr` is the first element (`arr[0]`)

17. Consider the following statement.

```java
int x = (int) (Math.random() * 8) - 3;
```

Describe all possible integer values that `x` can assume after this statement executes. In your answer, state the smallest and largest possible values and explain whether each endpoint is inclusive or exclusive based on how `Math.random()` and the cast to `int` behave.

a. It generates a random integer between -3 and 4, inclusive, and assigns it to `x`.
a. It always assigns the integer -3 to `x`.
a. It generates a random integer between -3 and 5, inclusive, and assigns it to `x`.
a. It generates a random integer between -2 and 5, inclusive, and assigns it to `x`.

18. Consider the following code segment.

```java
int x = 729;
while (x % 9 == 0)
{
    x /= 3;
}
System.out.println(x);
```

What is printed as a result of executing this code segment?


a. `3`
a. `1`
a. `729`
a. `9`

19. Consider the following code segment.

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

a. ```java
int k = 0; k < j; k++
```
a. ```java
int k = 0; k <= j; k++
```
a. ```java
int k = 0; k == j; k++
```
a. ```java
int k = 0; k < j; k--
```

20. The following class definition does not compile.

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

a. The `doSomething` method does not have access to variables named `s1` or `s2`.
a. The class has three instance variables (`a`, `b`, `c`), but its constructor has only two parameters (`s1`, `s2`).
a. The `doSomething` method is declared `static`, so it cannot access the instance variables `a` and `b`.
a. Assigning `a = s1` and `b = s2` in the constructor makes `s1` and `s2` become aliases for the fields, so `doSomething` can use `s1` and `s2`.

21. Consider the following method.

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


a. `{0, 0, 1, 0, 3}`
a. `{0, 1, 0, 3, 0}`
a. `{0, 0, 2, 0, 4}`
a. `{0, 0, 0, 0, 0}`

22. Consider the following code segment.

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

a. `[40, 10, 50]`
a. `[40, 10, 50, 20]`
a. `[40, 50, 20]`
a. `[40, 30, 50]`

23. Consider the following class definition.

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

a. 3
a. 4
a. 2
a. 1

24. A researcher wants to investigate whether students who took an engineering elective were more likely to participate in the school's robotics team. The researcher has access to the following data sets.

- Data set 1 contains an entry for each club at the school. Each entry includes the club name and a list of the names of the club members.
- Data set 2 contains an entry for each member of the robotics team. Each entry contains the member's name, the number of competitions the member has attended, and the member's favorite elective.
- Data set 3 contains an entry for each student at the school. Each entry contains the student's name, a list of the courses the student took, and a list of the clubs the student participated in.
- Data set 4 contains an entry for each student at the school. Each entry contains the student's name, the student's favorite elective, and the student's favorite club.

Which data set is most appropriate for the researcher's investigation? Explain your choice and briefly state why the other data sets are less suitable.

a. Data set 3 — It lists, for each student, the courses taken and the clubs participated in, allowing direct comparison of robotics participation between students who did and did not take an engineering elective.
a. Data set 2 — It lists each robotics team member and their favorite elective, but since it covers only robotics members you cannot compare participation rates between students who did and did not take engineering electives.
a. Data set 4 — It lists each student's favorite elective and favorite club, but 'favorite' does not equal courses taken or actual membership, so it cannot identify who took an engineering elective or who joined the robotics team.
a. Data set 1 — It lists each club and its members, so you can identify robotics participants, but it does not include the courses students took, so you cannot determine who took engineering electives.

25. Consider the following method.

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

a. `27 -13 7`
a. `7 -13 27`
a. `29 -15 7`
a. `28 -12 8`

26. The following incomplete method is intended to return the median (middle) value among its three parameters (i.e., the value that is neither the maximum nor the minimum).

```java
public static int mystery(int a, int b, int c)
{
    /* missing code */
}
```

Which of the following can be used to replace `/* missing code */` so that the method works as intended?

a. ```java
if ((a <= b && a >= c) || (a >= b && a <= c))
    return a;
else if ((b <= a && b >= c) || (b >= a && b <= c))
    return b;
else
    return c;
```
a. ```java
if ((a <= b || a >= c) || (a >= b || a <= c))
    return a;
else if ((b <= a || b >= c) || (b >= a || b <= c))
    return b;
else
    return c;
```
a. ```java
if ((a < b && a > c) || (a > b && a < c))
    return a;
else if ((b < a && b > c) || (b > a && b < c))
    return b;
else
    return c;
```
a. ```java
if (a <= b && b <= c)
    return b;
else if (b <= a && a <= c)
    return a;
else
    return c;
```

27. Consider the following code segment.

```java
int[][] mat = {{1, 2, 3},
               {4, 5, 6},
               {7, 8, 9}};
ArrayList<Integer> list = new ArrayList<Integer>();

/* missing code */
```

After executing this code segment, `list` should contain `[7, 4, 1, 8, 5, 2, 9, 6, 3]`. Which of the following can replace `/* missing code */` to produce this result?

a. ```java
for (int k = 0; k < mat[0].length; k++)
{
    for (int j = mat.length - 1; j >= 0; j--)
    {
        list.add(mat[j][k]);
    }
}
```
a. ```java
for (int k = 0; k < mat[0].length; k++)
{
    for (int j = 0; j < mat.length; j++)
    {
        list.add(mat[j][k]);
    }
}
```
a. ```java
for (int k = 0; k < mat[0].length; k++)
{
    for (int j = mat.length - 1; j >= 0; j--)
    {
        list.add(mat[k][j]);
    }
}
```
a. ```java
for (int k = 0; k < mat[0].length; k++)
{
    for (int j = 0; j < mat.length; j++)
    {
        list.add(0, mat[j][k]);
    }
}
```

28. Consider the following code segment.

```java
int[] arr = {3, 6, 9, 12, 15, 18, 21};

for (int i = 0; i < arr.length; i += 2)
{
    arr[i] += i;
}
```

What are the contents of `arr` after executing this code segment?

a. `{3, 6, 11, 12, 19, 18, 27}`
a. `{3, 7, 11, 15, 19, 23, 27}`
a. `{4, 6, 10, 12, 16, 18, 22}`
a. `{3, 6, 11, 12, 19, 18, 21}`

29. Consider the following code segment.

```java
ArrayList<String> listA = new ArrayList<String>();
listA.add("p");
listA.add("q");
listA.add("r");
listA.add("q");
ArrayList<String> listB = new ArrayList<String>();
/* missing code */
```

After executing this code segment, `listB` should contain $["q","r","q","p","p","q","r","q"]$. Which of the following can replace `/* missing code */` to produce this result?

a. ```java
for (int i = listA.size() - 1; i >= 0; i--)
{
    listB.add(listA.get(i));
}
for (int i = 0; i < listA.size(); i++)
{
    listB.add(listA.get(i));
}
```
a. ```java
for (int i = listA.size() - 1; i > 0; i--)
{
    listB.add(listA.get(i));
}
for (int i = 0; i < listA.size(); i++)
{
    listB.add(listA.get(i));
}
```
a. ```java
for (int i = listA.size() - 1; i >= 0; i--)
{
    listB.add(listA.get(i));
}
for (int i = 1; i < listA.size(); i++)
{
    listB.add(listA.get(i));
}
```
a. ```java
for (int i = listA.size() - 1; i >= 0; i--)
{
    listB.add(listA.get(i));
}
for (int i = 0; i < listA.size(); i++)
{
    listB.add(0, listA.get(i));
}
```

30. Consider the following method.

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

a. It returns `true` if `list` is sorted from least to greatest and returns `false` otherwise.
a. It returns `true` if `list` is strictly increasing (each element greater than the previous) and returns `false` otherwise.
a. It returns `true` if `list` is sorted from greatest to least and returns `false` otherwise.
a. It returns `true` if every element in `list` is greater than or equal to the minimum element in `list` and returns `false` otherwise.

31. Consider the following code segment.

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

a. ```
{{0, 0, 0},
 {0, 0, 3},
 {0, 2, 0},
 {1, 0, 0}}
```
a. ```
{{0, 0, 0},
 {0, 0, 0},
 {0, 0, 3},
 {0, 2, 0}}
```
a. ```
{{0, 0, 0},
 {0, 0, 2},
 {0, 1, 0},
 {0, 0, 0}}
```
a. ```
{{0, 0, 0},
 {0, 0, 3},
 {0, 2, 0},
 {0, 0, 0}}
```

32. Consider the following method. When an element of the two-dimensional array `arr` is accessed, the first index is used to specify the row and the second index is used to specify the column.

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

a. The number of times that an element equal to `target` appears in column `x` of `arr`
a. The number of times that an element equal to `target` appears in row `x` of `arr`
a. The number of rows in `arr` that contain column `x`
a. The number of times that an element equal to `target` appears anywhere in `arr`

33. The following method implements a recursive binary search variant.

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

a. `1`
a. `0`
a. `2`
a. `3`

34. Consider the following method that implements insertion sort on an ArrayList of Integer objects.

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

a. `[15, 25, 35, 45, 55, 5]`
a. `[45, 35, 25, 15, 55, 5]`
a. `[15, 35, 45, 25, 55, 5]`
a. `[15, 25, 55, 55, 55, 5]`

35. Consider the following code segment.

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

a. 5
a. 1
a. 4
a. 6

1. Consider the following code segment.

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

a. ```java
int s = 0;
for (int j = 1; j <= 6; j++)
{
    s += j;
}
```
a. ```java
int s = 0;
for (int j = 2; j <= 6; j++)
{
    s += j;
}
```
a. ```java
int s = 0;
for (int j = 1; j <= 7; j++)
{
    s += j;
}
```
a. ```java
int s = 0;
for (int j = 0; j < 6; j++)
{
    s += j;
}
```
