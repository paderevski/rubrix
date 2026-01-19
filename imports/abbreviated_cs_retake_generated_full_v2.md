1. Consider the following code segment.

```java
int a = 5;
int b = 3;
int c = a + b;
a = b * c;
b = c - a;
System.out.println(a + " " + b + " " + c);
```

What is printed as a result of executing this code segment?

a. `24 -16 8`
a. `24 3 8`
a. `24 16 8`
a. `11 -3 8`

2. In the following statement, $t_1$ and $t_2$ are properly declared and initialized `int` variables representing daily temperature deviations (in $^{\circ}\mathrm{C}$):

```java
boolean stable = ((t1 >= 0) && (t2 >= 0)) || ((t1 <= 0) && (t2 <= 0));
```

Which of the following best describes the conditions in which `stable` is assigned the value `true`?

a. **When `t1` and `t2` are both nonnegative or both nonpositive (i.e., they have the same sign or one/both are zero).**
a. **When `t1` and `t2` are both positive or both negative (zeros excluded).**

```java
boolean stable = ((t1 > 0) && (t2 > 0)) || ((t1 < 0) && (t2 < 0));
```
a. **When `t1` and `t2` are equal (i.e., `t1 == t2`).**
a. **When at least one of `t1` or `t2` is zero.**

3. Consider the following `Novella` class.

```java
public class Novella
{
    private String title;
    private String author;
    private int pages;

    public Novella(String t, int p)
    {
        title = t;
        pages = p;
        author = "Anonymous";
    }
    // There are no other constructors.
}
```

Which of the following code segments, appearing in a class other than `Novella`, will correctly create an instance of a `Novella` object?

a. ```java
Novella z = new Novella("Some Title", 150);
```
a. ```java
Novella x = new Novella(150, "Some Title");
```
a. ```java
Novella x = new Novella(Some Title, 150);
```
a. ```java
Novella x = Novella("Some Title", 150);
```

4. Consider the following code segment.

```java
int mystery = 7 + 6 / 3;
mystery = mystery - mystery % 4;
```

What is the value of `mystery` after this code segment is executed?

a. `mystery = 8`
a. `mystery = 4`
a. `mystery = 7`
a. `mystery = 10`

5. Consider the following code segment.

```java
double weight = 3.5;
double fee = 0.0;

if (weight >= 10.0)
{
    fee = 25.0;
}
else if (weight >= 5.0)
{
    fee = 12.0;
}
else if (weight >= 1.0)
{
    fee = 5.0;
}
else
{
    fee = 2.5;
}
System.out.println(weight + fee);
```

What is printed as a result of executing this code segment?

a. `8.5`
a. `28.5`
a. `8.0`
a. `5.0`

6. Consider the following code segment.

```java
for (int k = 0; k <= 10; k += 2) // Line 1
{
    // Line 2
    System.out.print(k + 1);
    // Line 3
}
// Line 4
```

This code segment is intended to produce only the following output. It does not work as intended.

`13579`

Which change(s) can be made so that this code segment works as intended?

a. In line 1, change the condition `k <= 10` to `k < 10` so the loop header becomes:

```java
for (int k = 0; k < 10; k += 2)
```
a. In line 1, change the initialization to `k = 1` so the loop header becomes:

```java
for (int k = 1; k <= 10; k += 2)
{
    // Line 2
    System.out.print(k + 1);
    // Line 3
}
```

This produces the output `246810`. (error: Wrong loop initialization (start at 1 instead of 0...)
a. In line 2, change the expression `k + 1` to `k` so the print statement becomes:

```java
System.out.print(k);
``` (error: Printing the wrong expression (print k instead of ...))
a. In line 1, change the increment `k += 2` to `k++` so the loop header becomes:

```java
for (int k = 0; k <= 10; k++)
```

This prints: `1234567891011` (error: Wrong increment operator in loop header (k++ inste...))

7. Consider the following code segment.

```java
String original = "STUDYNOW";
String result = /* missing code */;
System.out.println(result);
```

Which expression using calls to `substring` can replace `/* missing code */` so that this code segment prints the string "NOWSTU"?

a. ```java
original.substring(5) + original.substring(0, 3)
```
a. ```java
s.substring(0, 3) + s.substring(5)
```
a. ```java
s.substring(6) + s.substring(0, 3)
```
a. ```java
s.substring(4) + s.substring(0, 3)
```

8. Consider the following Java code segment.

```java
int p = -5;
double q = 2.0;
double r = (int)(p / q);
double s = (int)p / q;
System.out.println(r + " " + s);
```

What is printed as a result of executing this code segment?

a. `-2.0 -2.5`
a. `-3.0 -2.5`
a. `-2.0 -2.0`
a. `-2.5 -2.5`

9. Consider the following class definitions.

```java
public class Box
{
    private int length;
    private int width;
    private int height;

    public Box(int l, int w, int h)
    {
        length = l;
        width = w;
        height = h;
    }

    private int volume()
    {
        return length * width * height;
    }
}

public class BoxUtils
{
    public static boolean compare(Box b1, Box b2)
    {
        if (b1.volume() > b2.volume())
        {
            return true;
        }
        return false;
    }
}
```

Which of the following best explains why an error occurs when the classes are compiled?

a. The `volume` method cannot be accessed from a class other than `Box`.
a. A `static` method cannot call instance methods, so the `compare` method (declared `static`) cannot call `obj1.m()` or `obj2.m()`.
a. The instance `m` method cannot be accessed because `UtilsClass` is in a different package; the student assumes `m()` must be made package-private or `public` to be callable.
a. The `compare` method might not return a value on all execution paths.

10. Consider the following class declaration.

```java
public class Account
{
    private double balance;
    private String owner;

    public Account(double initial, String name)
    {
        balance = initial;
        owner = name;
    }

    public double checkBalance()
    {
        return balance;
    }
}
```

Assume that `myAcct` is a properly declared and initialized `Account` object that appears in a class other than `Account`. Which of the following code segments will compile without error?

(A) `double bal = myAcct.checkBalance();`

(B) `double bal = myAcct.checkBalance(50.0);`

(C) `double bal = myAcct.balance;`

(D) `double bal = myAcct.currentBalance;`

a. `double bal = myAcct.checkBalance();`
a. `double bal = myAcct.checkBalance(50.0);`
a. `double bal = myAcct.balance;`
a. `double bal = myAcct.checkBalance;`

11. Consider the following Java class:

```java
public class Coordinate {
    private int z;

    public Coordinate(int z) {
        this.z = z;
    }

    public int getZ() {
        return z;
    }
}
```

Which method declaration could be added to `Coordinate` so that code in other classes can change the value of $z$? Write the exact method (signature and body) you would add, and give a one-sentence explanation of why your method allows modification while a private method or a method that only returns the parameter would not.

a. ```java
public void m(int n) {
    this.z = n;
}
```
This method is public and assigns the parameter to the private field `z`, allowing other classes to call it and change `z`; a private method would not be callable from other classes and a method that only returns the parameter would not modify the field.
a. ```java
public void m(int z) {
    z = z;
}
```
This method is public but assigns the parameter to itself rather than to the instance field `z`, so calling it does not change the object's `z` value.
a. ```java
private void m(int n) {
    this.z = n;
}
```
This method assigns the parameter to `z` but is `private`, so other classes cannot call it to change `z`.
a. ```java
public int m(int n) {
    return n;
}
```
This method is public so other classes can call it, but it only returns the incoming value and does not assign to the private field `z`, so it does not change `z`.

12. Consider the following code segment.

```java
for (int a = 2; a <= 8; a += 3)
{
    for (int b = 5; b > 0; b -= 2)
    {
        System.out.println("#");
        // How many times is this statement executed?
    }
}
```

How many times will the statement in the comment line be executed as a result of running this code segment?

a. **9**
a. **6**
a. **12**
a. **21**

13. Assume that `isWeekend`, `hasWork`, and `hasTickets` are boolean variables that have been properly declared and initialized. Which expression is logically equivalent to the expression

$$\neg(\text{isWeekend} \lor \text{hasWork}) \land \text{hasTickets}$$

(You may write the equivalent using typical programming operators `!`, `&&`, and `||`.)

a. `!isWeekend && !hasWork && hasTickets`
a. `(!a || !b) && c`
a. `!a && !b || c`
a. `(!a && !b) || c`

14. Consider the following code segment.

```java
String str = "mississippi";
String target = "iss";
int j = str.indexOf(target);

while (j >= 0)
{
    str = str.substring(j);
    System.out.print(str + " ");
    str = str.substring(2);
    j = str.indexOf(target);
}
```

What is printed as a result of executing this code segment?

a. `ississippi issippi`
a. `ssissippi issippi`
a. `ississippi issippi ippi i`
a. `ississippi ippi`

15. The following method is intended to return the minimum value that appears in the `int` array `numbers`.

```java
public static int findMinimum(int[] numbers)
{
    int min = 0;
    for (int num : numbers)
    {
        if (num < min)
        {
            min = num;
        }
    }
    return min;
}
```

The method works as intended for some, but not all, `int` arrays. Which of the following best describes the conditions in which the method does not return the intended result?

a. When the minimum value in `numbers` is positive (greater than 0).
a. When the minimum value in `numbers` is negative (less than 0).
a. The method fails for any array whose minimum is not `Integer.MIN_VALUE` — it will always return `Integer.MIN_VALUE`.
a. When the minimum value in `numbers` is the last element (final position) of the array.

16. Consider the following Java statement:

```java
int foo = (int)(Math.random() * 8) - 3;
```

Which of the following best describes the set of possible integer values that can be assigned to `foo`?

a. It generates a random integer between -3 and 4, inclusive, and assigns it to `foo`.
a. It generates a random integer between -3 and 5, inclusive, and assigns it to `foo`.
a. It generates a random integer between 0 and 4, inclusive, and assigns it to `foo`.
a. It generates a random integer between -2 and 5, inclusive, and assigns it to `foo`.

17. Consider the following code segment.

```java
int mystery = 50;
do {
    mystery /= 3;
} while (mystery % 2 == 0);
System.out.println(mystery);
```

What is printed as a result of executing this code segment?

a. `5`
a. `16`
a. `17`
a. `16.666...`

18. Consider the following code segment.

```java
for (int r = 5; r > 0; r--)
{
    for (/* missing code */)
    {
        System.out.print("*");
    }
    System.out.println();
}
```

This code segment is intended to produce the following output.

```
*****
****
***
**
*
```

Which of the following can be used to replace `/* missing code */` so that this code segment works as intended?

a. `int q = 0; q < r; q++`
a. `int q = 1; q < r; q++`
a. `int q = 0; q <= r; q++`
a. `int q = 0; q < r; r--`

19. The following class definition does not compile.

```java
public class Vehicle
{
    private String make;
    private String model;
    private int year;

    public Vehicle(String vMake, String vModel, int vYear)
    {
        make = vMake;
        model = vModel;
        year = vYear;
    }

    public void advanceYear()
    {
        year++;
    }

    public String getDisplay()
    {
        return makeName + " " + modelName + " (" + vYear + ")";
    }
}
```

Which of the following best explains why this class will not compile?

a. The `getDisplay` method references variables that do not exist (`makeName`, `modelName`, and `vYear`) instead of the class fields (`make`, `model`, and `year`).
a. The `getDisplay` method cannot access the instance variables because they are declared `private`; the fields should be declared `public`.
a. The constructor defines `paramMake`, `paramModel`, and `paramYear`, which the student assumes are the object's fields; the class fails to compile because the methods use different names (`fieldMake`, `fieldModel`, `fieldYear`) instead of those constructor parameter names.
a. The class will compile because the student assumes property-like names are auto-generated: `propMake` and `propModel` refer to the `fieldMake` and `fieldModel` fields, so `getDisplay` is using valid property names.

20. Consider the following method.

```java
public static void mirrorAssign(int[] arr, int pos) {
    arr[arr.length - 1 - pos] = pos;
}
```

The following code segment appears in a method in the same class as `mirrorAssign`.

```java
int[] data = new int[5];
mirrorAssign(data, 1);
mirrorAssign(data, 3);
mirrorAssign(data, 4);
```

What are the contents of `data` after executing this code segment?

a. `{4, 3, 0, 1, 0}`
a. `{0, 1, 0, 3, 4}`
a. `{0, 4, 3, 0, 1}`
a. `{0, 3, 0, 1, 0}`

21. Consider the following code segment.

```java
ArrayList<String> people = new ArrayList<String>();
people.add("Amy");
people.add("Ben");
people.add(1, "Cara");
people.set(0, "Zoe");
people.add(2, "Liam");
```

What are the contents of `people` after executing this code segment?

a. `[Zoe, Cara, Liam, Ben]`
a. `[Zoe, Cara, Liam]`
a. `[Zoe, Liam, Amy, Ben]`
a. `[Zoe, Cara, Ben, Liam]`

22. Consider the following class definition.

```java
public class Backpack
{
    private static int total = 0;
    private int capacity;
    private String color;

    public Backpack()
    {
        capacity = 20;
        color = "black";
        total++;
    }

    public Backpack(int cap, String col)
    {
        capacity = cap;
        color = col;
    }

    public static int getTotal()
    {
        return total;
    }
}
```

The following code segment appears in a class other than `Backpack`.

```java
Backpack b1 = new Backpack();
Backpack b2 = new Backpack(30, "red");
Backpack b3 = new Backpack();
Backpack b4 = new Backpack(25, "blue");
System.out.println(Backpack.getTotal());
```

What is printed as a result of executing this code segment if it is executed before any other instances of `Backpack` are constructed?

a. `2`
a. `4`
a. `3`
a. `1`

23. A university researcher wants to determine whether students who took AP Physics were more likely to participate on the university robotics team. The researcher has access to several existing data sets.

- Data set 1 contains one record per campus club. Each record lists the club name and the total number of members.
- Data set 2 contains one record per robotics team member. Each record lists the member's name, the number of competitions they attended, and their favorite course.
- Data set 3 contains one record per student. Each record lists the student's name, the full list of courses the student completed, and the list of extracurricular teams or clubs the student joined.
- Data set 4 contains one record per student. Each record lists the student's name, cumulative GPA, and declared major.

Which single data set would be most appropriate for answering the research question about whether taking AP Physics is associated with joining the robotics team?

a. Data set 3
a. Data set 2
a. Data set 1
a. Data set 4

24. Consider the following method.

```java
public static void foo(int n)
{
    if (Math.abs(n) < 100)
    {
        foo(n * -3);
        System.out.print(n + " ");
    }
}
```

What is printed as a result of the call `foo(2)`?


a. `-54 18 -6 2`
a. `2 -6 18 -54`
a. `54 18 6 2`
a. `162 -54 18 -6 2`

25. The following incomplete method is intended to return the middle (median) value among its three parameters (i.e., the value that is neither the maximum nor the minimum).

```java
public static int medianOfThree(int a, int b, int c)
{
    /* missing code */
}
```

Which of the following can be used to replace `/* missing code */` so that the method works as intended?

a. ```java
if ((a >= b && a <= c) || (a <= b && a >= c))
    return a;
else if ((b >= a && b <= c) || (b <= a && b >= c))
    return b;
else
    return c;
```
a. ```java
if ((a > b && a < c) || (a < b && a > c))
    return a;
else if ((b > a && b < c) || (b < a && b > c))
    return b;
else
    return c;
```
a. ```java
if ((a >= b && a <= c) && (a <= b && a >= c))
    return a;
else if ((b >= a && b <= c) && (b <= a && b >= c))
    return b;
else
    return c;
```
a. ```java
return a + b + c - Math.max(a,b) - Math.min(a,b);
```

26. Consider the following code segment.

```java
int[][] grid = {{1, 2},
                {3, 4},
                {5, 6}};
ArrayList<Integer> result = new ArrayList<Integer>();

/* missing code */

```

After executing this code segment, `result` should contain `[5, 3, 1, 6, 4, 2]`. Which of the following can replace `/* missing code */` to produce this result?

a. ```java
for (int c = 0; c < grid[0].length; c++)
{
    for (int r = grid.length - 1; r >= 0; r--)
    {
        result.add(grid[r][c]);
    }
}
```
a. ```java
for (int i = grid.length - 1; i >= 0; i--)
{
    for (int j = 0; j < grid[0].length; j++)
    {
        result.add(grid[i][j]);
    }
}
```
a. ```java
for (int j = 0; j < grid[0].length; j++)
{
    for (int i = 0; i < grid.length; i++)
    {
        result.add(grid[i][j]);
    }
}
```
a. ```java
for (int j = grid[0].length - 1; j >= 0; j--)
{
    for (int i = grid.length - 1; i >= 0; i--)
    {
        result.add(grid[i][j]);
    }
}
```

27. Consider the following code segment.

```java
int[] arr = {3, 6, 9, 12, 15};

for (int k = arr.length / 2; k >= 0; k -= 2)
{
    arr[k] *= 2;
}
```

What are the contents of `arr` after executing this code segment?

a. `{6, 6, 18, 12, 15}`
a. `{3, 6, 18, 12, 15}`
a. `{6, 12, 18, 12, 15}`
a. `{2, 6, 2, 12, 15}`

28. Consider the following code segment.

```java
ArrayList<Integer> oldList = new ArrayList<Integer>();
oldList.add(10);
oldList.add(20);
oldList.add(30);
ArrayList<Integer> newList = new ArrayList<Integer>();
/* missing code */
```

After executing this code segment, `newList` should contain $[30, 20, 10, 10, 20, 30]$. Write a replacement for `/* missing code */` (using a loop or loops and the `add` methods of `ArrayList`) that produces this result. Explain briefly why your code produces the required ordering.


a. ```java
ArrayList<Integer> p = oldList;
ArrayList<Integer> q = newList;
for (int i = p.size() - 1; i >= 0; i--) {
    q.add(p.get(i));
}
for (int i = 0; i < p.size(); i++) {
    q.add(p.get(i));
}
```

First loop adds the elements of `oldList` in reverse order (30, 20, 10). The second loop then appends the elements in forward order (10, 20, 30), producing `[30, 20, 10, 10, 20, 30]`.
a. ```java
for (int i = oldList.size() - 1; i > 0; i--) {
    newList.add(oldList.get(i));
}
for (int i = 0; i < oldList.size(); i++) {
    newList.add(oldList.get(i));
}
```
a. ```java
for (int i = oldList.size() - 1; i >= 0; i--) {
    newList.add(oldList.get(i));
}
for (int i = 0; i < oldList.size() - 1; i++) {
    newList.add(oldList.get(i));
}
```
a. ```java
for (int i = 0; i < oldList.size(); i++) {
    newList.add(oldList.get(i));
}
for (int i = oldList.size() - 1; i >= 0; i--) {
    newList.add(oldList.get(i));
}
```

29. Consider the following method.

```java
public static boolean mystery(ArrayList<String> words)
{
    String prev = null;
    for (String w : words)
    {
        if (prev != null && w.compareTo(prev) < 0)
        {
            return false;
        }
        prev = w;
    }
    return true;
}
```

Which of the following best describes the behavior of the method?

a. It returns `true` if `words` is sorted from least to greatest (non-decreasing lexicographic order) and returns `false` otherwise.
a. It returns `true` if `words` is sorted from least to greatest with each element strictly greater than the previous (strictly increasing) and returns `false` otherwise.
a. It throws a `NullPointerException` on the first iteration because it attempts to call `compareTo` with a `null` previous value.
a. It returns `true` if every element in `words` is lexicographically greater than or equal to the first element (i.e., all elements >= the first); it returns `false` otherwise.

30. Consider the following code segment.

```java
int[][] grid = new int[3][5];

for (int r = 0; r < grid.length; r++) {
    for (int c = 0; c < grid[0].length; c++) {
        if (r == c) {
            grid[r][c] = c;
        }
    }
}
```

What are the contents of `grid` after executing this code segment? Write your answer as a 2D array using nested braces, for example `{{...}, {...}, {...}}`.


a. ```
{{0, 0, 0, 0, 0},
 {0, 1, 0, 0, 0},
 {0, 0, 2, 0, 0}}
```
a. ```
{{0, 1, 2, 3, 4},
 {0, 1, 2, 3, 4},
 {0, 0, 2, 3, 4}}
```
a. ```
{{0, 0, 0, 0, 0},
 {0, 1, 0, 0, 0},
 {0, 0, 4, 0, 0}}
```
a. ```
{{0, 0, 0},
 {0, 1, 0},
 {0, 0, 2},
 {0, 0, 0},
 {0, 0, 0}}
```

31. Consider the following method. When an element of the two-dimensional array `matrix` is accessed, the first index is used to specify the row and the second index is used to specify the column.

```java
public static int mystery(int[][] matrix, int col, int key)
{
    int count = 0;
    for (int r = 0; r < matrix.length; r++)
    {
        if (matrix[r][col] == key)
        {
            count++;
        }
    }
    return count;
}
```

Which of the following best describes the value returned by the method?

a. **The number of times that an element equal to `key` appears in column `col` of `matrix`.**
a. **The number of times that an element equal to `k` appears in row `c` of `m`.**
a. **The number of times that an element equal to `k` appears anywhere in `m`.**
a. **The number of times that an element equal to `k` appears in column `c` of `m`, plus one.**

32. The following method implements a recursive binary search algorithm.

```java
public static int binarySearch(int[] elements, int low, int high, int target)
{
    if (low <= high)
    {
        int mid = (high + low) / 2;
        if (target < elements[mid])
        {
            return binarySearch(elements, low, mid - 1, target);
        }
        else if (target > elements[mid])
        {
            return binarySearch(elements, mid + 1, high, target);
        }
        else if (elements[mid] == target)
        {
            return mid;
        }
    }
    return -1;
}
```

The following code segment appears in a method in the same class as `binarySearch`.

```java
int[] values = {3, 3, 6, 6, 6, 7, 8, 8, 8, 10, 12};
int mystery = binarySearch(values, 2, 8, 6);
```

What is the value of `mystery` after executing the code segment?

a. `3`
a. `6`
a. `4`
a. `2`

33. The following method is a correct implementation of the insertion sort algorithm. The method correctly sorts the elements of `numList` so that they appear in order from least to greatest.

```java
public static void insertionSort(ArrayList<Integer> numList)
{
    for (int j = 1; j < numList.size(); j++)
    {
        int temp = numList.get(j);
        int k = j;
        while (k > 0 && temp < numList.get(k - 1))
        {
            numList.set(k, numList.get(k - 1));
            k--;
        }
        numList.set(k, temp);
        /* end of outer loop */
    }
}
```

Assume that `insertionSort` has been called with an `ArrayList` parameter that has been initialized with the following Integer objects:

`[45, 15, 30, 20, 50, 10]`

What will the contents of `numList` be after three passes of the outer loop (i.e., when $j == 3$ at the point indicated by `/* end of outer loop */`)?

a. `[15, 20, 30, 45, 50, 10]`
a. `[15, 30, 45, 20, 50, 10]`
a. `[15, 30, 20, 45, 50, 10]`
a. `[15, 20, 30, 20, 50, 10]`

34. Consider the following code segment.

```java
String[] objects = {"sun", "moon", "star", "sky", "comet", "galaxy"};
int mystery = -1;

for (int i = 0; i < objects.length; i++)
{
    for (int j = i + 1; j < objects.length; j++)
    {
        if (objects[i].length() == objects[j].length())
        {
            mystery = j;
        }
    }
}
```

What is the value of `mystery` after executing this code segment?

a. `2`
a. `3`
a. `1`
a. `4`

35. Consider the following code segment.

```java
int k = 0;
int total = 0;
while (k < 7)
{
    k++;
    total += k * k;
}
```

Which of the following code segments assigns the same value to `total` as the preceding code segment?

a. ```java
int total = 0;
for (int p = 1; p <= 7; p++)
{
    total += p * p;
}
```
a. ```java
int total = 0;
for (int p = 1; p < 7; p++)
{
    total += p * p;
}
```
a. ```java
int total = 0;
for (int p = 0; p < 7; p++)
{
    total += p * p;
}
```
a. ```java
int total = 0;
for (int p = 1; p <= 7; p++)
{
    total = p * p;
}
```
