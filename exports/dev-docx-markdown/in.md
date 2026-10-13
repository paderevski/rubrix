# Quiz

::: {custom-style="QuestionBody"}
**Question 1.** Examine the following Java code.
:::
```java
int x = 8;
int y = 4;
int hold = x;
x = y - 1;
y = hold + x;
System.out.println(x + " " + y);
```
What is printed when this code runs?

A) 3 7
B) 5 13
C) 3 8
D) 15 16
E) 3 11

\hline
::: {custom-style="QuestionBody"}
**Question 2.** Assume int variables a and b have been properly declared and initialized. Consider the following statement:
:::
```java
boolean c = ((a % 2 == 0) && (b % 2 == 0)) || ((a % 2 != 0) && (b % 2 != 0));
```
Which of the following best describes the condition(s) under which c is assigned the value true?

A) When at least one of a or b is even.
B) When both a and b are even, or both a and b are odd.
C) When both a and b are multiples of 2.
D) When at least one of a and b is a mulitple of 2

\hline
::: {custom-style="QuestionBody"}
**Question 3.** Consider the following Painting class.
:::
```java
public class Painting {
    private int width;
    private String title;
    private int yearCreated;
    public Painting(int w, String t) {
        width = w;
        title = t;
        yearCreated = 0;
    }
    // No other constructors are defined.
}
```
Which of the following code fragments, written in a class other than Painting, will correctly create a Painting object?

A)
    ```java
    Painting p = new Painting("Starry Night", 72);
    ```
B)
    ```java
    Painting p = new Painting(72, "Starry Night", 1889);
    ```
C)
    ```java
    Painting p = new Painting(72.0, "Starry Night");
    ```
D)
    ```java
    Painting p = new Painting(72, "Starry Night");
    ```

\hline
::: {custom-style="QuestionBody"}
**Question 4.** Consider the following code segment.
:::
```java
int a = 8 + 3 * 4;
int b = a - a % 3;
```
What is the value of b after this code segment is executed?

A) 42
B) 0
C) 17
D) 18

::: {custom-style="QuestionBody"}
**Question 5.** Consider the following code segment.
:::
```java
double price = 72.5;
double coupon = 0.0;
if (price > 80.0) {
    coupon = 15.0;
} else if (price >= 60.0) {
    coupon = 10.0;
} else if (price > 30.0) {
    coupon = 5.0;
} else {
    coupon = 0.0;
}
System.out.println(price - coupon);
```
What is printed when this code segment is executed?

A) 72.5
B) 57.5
C) 67.5
D) 65.25
E) 62.5

::: {custom-style="QuestionBody"}
**Question 6.** Consider the following code segment.
:::
```java
for (int i = 5; i >= 1; i = i - 2) {
    System.out.print(i - 1);
}
```
This code segment is intended to produce only the following output:
```
43210
```
It does not work as intended. Which of the following single changes will make the code produce the intended output?

A) In the for‑loop header, change the update expression from `i = i - 2` to `i = i - 1`.
B) In the for‑loop header, change the loop condition from `i >= 1` to `i > 1`.
C) In the loop body, change the printed expression from `i - 1` to `i + 1`.
D) In the for‑loop header, change the initialization from `int i = 5` to `int i = 4`.

::: {custom-style="QuestionBody"}
**Question 7.** Consider the following code segment.
:::
```java
String original = "DEFGHI";
String result = /* missing code */;
System.out.println(result);
```
  Which of the following expressions can be used to replace /* missing code */ so that this
code segment prints the string "HIDE"?

A)
    ```java
    original.substring(4) + original.substring(0, 1)
    ```
B)
    ```java
    original.substring(4) + original.substring(0, 2)
    ```
C)
    ```java
    original.substring(5) + original.substring(1, 2)
    ```
D)
    ```java
    original.substring(5) + original.substring(1, 3)
    ```

::: {custom-style="QuestionBody"}
**Question 8.** Consider the following code segment.
:::
```java
double a = 5.0;
double b = 2.0;
double c = (int) a / b;
double d = (int) (a / b);
System.out.println(c + ", " + d);
```
What is printed when this code is executed?

A) 2.0, 2.0
B) 2.0, 2.5
C) 2.5, 2.5
D) 2.5, 2.0

::: {custom-style="QuestionBody"}
**Question 9.** Consider the following class definitions.
:::
```java
public class Counter {
    private int count;
    public Counter(int start) {
       count = start;
    }
    private int next() {
       count++;
       return count;
   }
}
public class Utils {
    public static int getNext(Counter c) {
        return c.next();
    }
}
```
Which statement best explains why the code fails to compile?

A) The method `next()`is private, so it cannot be accessed from the `Utils` class.
B) The `getNext` method should be declared void because it only retrieves a value.
C) The `Counter` class must be declared abstract to allow its methods to be called from another class.
D) The variable `count` is private, so any method that uses it must also be private.

::: {custom-style="QuestionBody"}
**Question 10.** A banking application should display a status message based on a customer's account balance.
:::

- "Overdrawn!" is displayed for balances less than 0.
- "Low balance" is displayed for balances from 0 up to (but not including) 100.
- "Good standing" is displayed for balances of 100 or more.

Assume **balance** is a properly declared and initialized **double** that represents the account balance.

Which of the following code segments will print the correct status message for any valid value of **balance**?

A)
    ```java
    String status = "";
    if (balance < 0) {
        status = "Overdrawn!";
    } else if (balance < 100) {
        status = "Low balance";
    } else {
        status = "Good standing";
    }
    System.out.println(status);
    ```
B)
    ```java
    String status = "";
    if (balance < 0) {
        status = "Overdrawn!";
    }
    if (balance < 100) {
        status = "Low balance";
    } else {
        status = "Good standing";
    }
    System.out.println(status);
    ```
C)
    ```java
    String status = "";
    if (balance <= 0) {
        status = "Overdrawn!";
    } else if (balance < 100) {
        status = "Low balance";
    } else {
        status = "Good standing";
    }
    System.out.println(status);
    ```
D)
    ```java
    String status = "";
    if (balance < 0) {
        status = "Overdrawn!";
    } else if (balance >= 0) {
        status = "Low balance";
    } else if (balance >= 100) {
        status = "Good standing";
    }
    System.out.println(status);
    ```

::: {custom-style="QuestionBody"}
**Question 11.** Consider the following class definition:
:::

```java
public class Circle {
    private int radius;
    private String color;

    public Circle(int r, String c) {
        radius = r;
        color = c;
    }

    public int getRadius() {
        return radius;
    }

    public double getArea() {
        return Math.PI * radius * radius;
    }
}
```

Assume that a properly declared and initialized Circle object named `myCircle` exists in a class other than `Circle`. Which of the following code fragments will compile without error?

A)
    ```java
    int r = myCircle.getRadius();
    ```
B)
    ```java
    int r = myCircle.getRadius(7);
    ```
C)
    ```java
    int r = myCircle.radius;
    ```
D)
    ```java
    double a = myCircle.GetArea();
    ```

::: {custom-style="QuestionBody"}
**Question 12.** The class below stores a point in 2‑D space. Which method definition would correctly allow other classes to change the value of the private instance variable **y**?
:::

```java
public class Coordinate {
    private int x;
    private int y;

    public Coordinate(int x, int y) {
        this.x = x;
        this.y = y;
    }
    // ... other methods ...
}
```

A)
    ```java
    private void setY(int newY) {
        y = newY;
    }
    ```
B)
    ```java
    public int setY(int newY) {
        return newY;
    }
    ```
C)
    ```java
    public void setY() {
        y = 0;
    }
    ```
D)
    ```java
    public void setY(int newY) {
        this.y = newY;
    }
    ```

::: {custom-style="QuestionBody"}
**Question 13.** Examine the following Java code segment.
:::
```java
for (int i = 1; i <= 4; i++) {
    for (int j = 0; j < i; j++) {
        System.out.print("#");
    }
}
```
How many times is the statement `System.out.print("#");` executed when this code runs?

A) 6
B) 12
C) 14
D) 10

::: {custom-style="QuestionBody"}
**Question 14.** Assume that isPrime, isOdd, and isPositive are boolean variables that have been properly declared and initialized. Which of the following expressions is equivalent to the expression
:::

    !(isPrime || isOdd) && isPositive ?

A) (!isPrime && !isOdd) && isPositive
B) (!isPrime || !isOdd) && isPositive
C) (!isPrime && !isOdd) && !isPositive
D) (!isPrime && isOdd) && isPositive

::: {custom-style="QuestionBody"}
**Question 15.** Consider the following Java code segment.
:::
```java
String s = "programming";
String t = "g";
int i = s.indexOf(t);
while (i >= 0) {
    s = s.substring(i);
    System.out.print(s + " ");
    s = s.substring(1);
    i = s.indexOf(t);
}
```
What is printed when this code segment is executed?

A) gramming g
B) gramming
C) ramming g
D) programming g

::: {custom-style="QuestionBody"}
**Question 16.** The Movie class must store a film’s title, director, and duration in minutes. Other classes should be able to read these values but must not be able to modify them directly. Which of the following class definitions satisfies this requirement?
:::

A)
    ```java
    public class Movie {
        public String title;
        public String director;
        public int duration;
        public String getTitle() { return title; }
        public String getDirector() { return director; }
        public int getDuration() { return duration; }
    }
    ```
B)
    ```java
    public class Movie {
        private String title;
        private String director;
        private double duration; // should be int
        public String getTitle() { return title; }
        public String getDirector() { return director; }
        public double getDuration() { return duration; }
    }
    ```
C)
    ```java
    public class Movie {
        private String title;
        private String director;
        private int duration;
        private String getTitle() { return title; }
        private String getDirector() { return director; }
        private int getDuration() { return duration; }
    }
    ```
D)
    ```java
    public class Movie {
        private String title;
        private String director;
        private int duration;
        public String getTitle() { return title; }
        public String getDirector() { return director; }
        public int getDuration() { return duration; }
    }
    ```

::: {custom-style="QuestionBody"}
**Question 17.** A developer is creating a mobile app that should run on many different devices and operating systems worldwide. Which of the following actions will most improve the app’s portability?
:::

A) Hard‑coding absolute file paths that reference the device’s internal storage layout.
B) Including a native library written in C that is compiled for a single processor architecture.
C) Using reflection to access private fields of system classes.
D) Using only standard Java APIs that are part of the Java SE platform and avoiding platform‑specific libraries.

::: {custom-style="QuestionBody"}
**Question 18.** The following method is intended to return the largest value stored in the
:::
`int` array `nums`.
```java
public static int findMax(int[] nums) {
    int max = nums[0];
    int i = 0;
    while (i < nums.length - 1) {
        if (nums[i] > max) {
            max = nums[i];
        }
        i++;
    }
    return max;
}
```
The method works for many arrays but fails for some. Which of the following best
describes the situation in which the method does NOT return the correct maximum
value?

A) When the largest value is stored in the last position of the array.
B) When the largest value appears more than once in the array.
C) When the largest value in the array is negative.
D) When the largest value is the first element of the array.

::: {custom-style="QuestionBody"}
**Question 19.** Consider the following statement.
:::
```java
int n = (int) (Math.random() * 7) + 3;
```
Which of the following best describes the behavior of the statement?

A) It generates a random integer between 3 and 10, inclusive, and assigns it to n.
B) It generates a random integer between 3 and 7, inclusive, and assigns it to n.
C) It generates a random integer between 0 and 9, inclusive, and assigns it to n.
D) It generates a random integer between 4 and 10, inclusive, and assigns it to n.
E) It generates a random integer between 3 and 9, inclusive, and assigns it to n.

::: {custom-style="QuestionBody"}
**Question 20.** Consider the following code segment.
:::
```java
int num = 72;
while (num % 3 == 0) {
    num = num / 2;
}
System.out.println(num);
```
What is printed as a result of executing this code segment?

A) 0
B) 4
C) 5
D) 36
E) 9

::: {custom-style="QuestionBody"}
**Question 21.** Consider the following code segment.
:::
```java
for (int i = 1; i <= 5; i++)
{
   for (/* missing code */)
   {
      System.out.print(i + " ");
   }
   System.out.println();
}
```
This code segment is intended to produce the following output.
1
2 2
3 3 3
4 4 4 4
5 5 5 5 5
Which of the following can be used to replace /* missing code */ so that this code segment works as intended?

A)
    ```java
    int k = 0; k < i; k++
    ```
B)
    ```java
    int k = 0; k <= i; k++
    ```
C)
    ```java
    int k = i; k < 5; k++
    ```
D)
    ```java
    int k = 0; k < i; k+=2
    ```
E)
    ```java
    int k = i; k <= 5; k++
    ```

::: {custom-style="QuestionBody"}
**Question 22.** The following class definition does not compile.
:::
```java
public class Car {
    private String make;
    private String model;
    private int year;

    public Car(String carMake, String carModel, int carYear) {
        make = carMake;
        model = carModel;
        year = carYear;
    }

    public void increaseYear() {
        year++;
    }

    public String getDescription() {
        return carMake + " " + carModel + ", " + carYear;
    }
}
```
Which of the following best explains why the class will not compile?

A) The constructor provides three arguments, but the class only declares two instance variables.
B) The instance variables should be declared public instead of private.
C) The increaseYear method should return an int instead of void.
D) The getDescription method refers to variables named carMake, carModel, and carYear that are not in scope.

::: {custom-style="QuestionBody"}
**Question 23.** Examine the following method.
:::
```java
public static void assignValue(int[] data, int idx, int value) {
    data[idx] = value;
}
```
The code segment below is placed in the same class.
```java
int[] scores = new int[5];
assignValue(scores, 2, 7);
assignValue(scores, 4, 3);
assignValue(scores, 0, 9);
```
What are the contents of the array **scores** after this segment executes?

A) {0, 0, 0, 0, 0}
B) {9, 0, 7, 0, 3}
C) {0, 7, 0, 3, 0}
D) {0, 0, 2, 0, 4}

::: {custom-style="QuestionBody"}
**Question 24.** Consider the following code segment.
:::
```java
ArrayList<String> colors = new ArrayList<String>();
colors.add("Red");
colors.add("Blue");
colors.add(1, "Green");
colors.set(0, "Yellow");
colors.add(2, "Purple");
colors.remove(3);
```
What are the contents of **colors** after executing this code segment?

A) [Yellow, Green, Purple]
B) [Yellow, Green, Purple, Blue]
C) [Yellow, Purple, Green]
D) [Yellow, Green, Blue, Purple]

::: {custom-style="QuestionBody"}
**Question 25.** Consider the following class definition.
:::
```java
public class Pen {
    private static int total = 0;
    private String color;
    private int tipSize;

    public Pen() {
        color = "black";
        tipSize = 1;
    }

    public Pen(String c, int t) {
        color = c;
        tipSize = t;
        total++;
    }

    public static int getTotal() {
        return total;
    }
}
```
The following code segment appears in a class other than Pen.
```java
Pen p1 = new Pen("blue", 2);
Pen p2 = new Pen();
Pen p3 = new Pen("red", 1);
Pen p4 = new Pen("green", 3);
System.out.println(Pen.getTotal());
```
What is printed when this code segment is executed before any other Pen objects are created?

A) 0
B) 1
C) 3
D) 4

::: {custom-style="QuestionBody"}
**Question 26.** In the following code segment, `n` is a properly declared and initialized `int` variable whose value is positive.
:::

```java
while (n > 0) {        // Line 1
    n = n / 10;        // Line 2
}                       // Line 3
System.out.println(n); // Line 4
```

The code is intended to print the leftmost digit of `n`. For example, if `n` is `8347`, it should print `8`. However, the code does not work as intended. Which single change will make the code work correctly?

A) In line 1, change the condition `n > 0` to `n > 9`.
B) In line 1, change the condition `n > 0` to `n % 10 > 0`.
C) In line 2, change the statement `n = n / 10;` to `n = n - 10;`.
D) In line 2, change the statement `n = n / 10;` to `n = n % 10;`.

::: {custom-style="QuestionBody"}
**Question 27.** A data file stores one inventory record per line. Each line contains three fields separated by a single space: the item's name (a String), the quantity on hand (an int), and the unit price (a double). For example:
:::
```
Gadget 2000 3.75
```
Write a method `double totalValue(Scanner sc)` that reads the next line from the scanner and returns the product of the quantity and the unit price as a double. Which of the following code fragments will always return the correct product for a properly formatted line?

A)
    ```java
    String name = sc.next();
    int quantity = sc.nextInt();
    double price = sc.nextDouble();
    return quantity * price;
    ```
B)
    ```java
    String line = sc.nextLine();
    Scanner lineSc = new Scanner(line);
    lineSc.useDelimiter(",");
    String name = lineSc.next();
    int quantity = lineSc.nextInt();
    double price = lineSc.nextDouble();
    lineSc.close();
    return quantity * price;
    ```
C)
    ```java
    String name = sc.next();
    int quantity = sc.nextInt();
    int price = sc.nextInt();
    return quantity * price;
    ```
D)
    ```java
    String line = sc.nextLine();
    String[] parts = line.split(" ");
    int quantity = Integer.parseInt(parts[1]);
    double price = Double.parseDouble(parts[2]);
    return (int)quantity * (int)price;
    ```

::: {custom-style="QuestionBody"}
**Question 28.** Consider the following code segment.
:::
```java
String txt = "mississippi";
int k = 0;
int count = 0;
while (k < txt.length() - 2) {
    if (txt.substring(k, k + 1)
            .equals(txt.substring(k + 2, k + 3))) {
        count++;
    }
    k++;
}
System.out.println(count);
```
What is printed as a result of executing this code segment?

A) 0
B) 1
C) 3
D) 6

::: {custom-style="QuestionBody"}
**Question 29.** Consider the following method.
:::
```java
public static void recur(int v) {
    if (v >= 0) {
        recur(v - 5);
        System.out.print(v + " ");
    }
}
```
What is printed as a result of the call recur(12)?

A) 2 7 12
B) 12 7 2
C) -3 2 7 12
D) 2 7 12 17

::: {custom-style="QuestionBody"}
**Question 30.** The following incomplete method should return the largest of its three integer parameters.
:::
```java
public static int maxOfThree(int x, int y, int z) {
    /* missing code */
}
```
Which of the following code fragments can replace the comment so that the method works correctly?

A)
    ```java
    if (x >= y && x >= z) {
        return x;
    } else if (y >= x && y >= z) {
        return y;
    } else {
        return z;
    }
    ```
B)
    ```java
    if (x >= y || x >= z) {
        return x;
    } else if (y >= x || y >= z) {
        return y;
    } else {
        return z;
    }
    ```
C)
    ```java
    if (x > y && x > z) {
        return x;
    } else if (y > x && y > z) {
        return y;
    } else {
        return z;
    }
    ```
D)
    ```java
    if (x >= y) {
        if (x >= z) {
            return x;
        } else {
            return y;
        }
    } else {
        if (y >= z) {
            return y;
        } else {
            return z;
        }
    }
    ```

::: {custom-style="QuestionBody"}
**Question 31.** A game studio is planning its next expansion and wants to decide which new gameplay elements to develop first. To gather opinions, the studio emails a survey to every player who logged in at least 200 times during the last six months. The survey asks participants to rate each proposed element on a scale of 1‑5. After the responses are collected, a data analyst will compute the average rating for each element and recommend the highest‑scoring ones to the development team. Which of the following statements most accurately explains why the analyst should be cautious about drawing conclusions from this survey data?
:::

A) The survey data may be biased because it only includes responses from the most active players.
B) The survey data may be unreliable because it includes responses from players who no longer play the game.
C) The survey data may raise intellectual‑property concerns that could affect the analysis.
D) The survey data may need to be encapsulated in objects before it can be analyzed.

::: {custom-style="QuestionBody"}
**Question 32.** Consider the following code segment.
:::
```java
int[][] grid = {
    {1, 2, 3, 4},
    {5, 6, 7, 8},
    {9, 10, 11, 12}
};
ArrayList<Integer> result = new ArrayList<Integer>();
/* missing code */
```
After executing this code segment, result should contain [4, 8, 12, 3, 7, 11, 2, 6, 10, 1, 5, 9]. Which of the following can replace /* missing code */ to produce this result?

A)
    ```java
    for (int c = grid[0].length - 1; c >= 0; c--) {
        for (int r = 0; r < grid.length; r++) {
            result.add(grid[r][c]);
        }
    }
    ```
B)
    ```java
    for (int c = 0; c < grid[0].length; c++) {
        for (int r = 0; r < grid.length; r++) {
            result.add(grid[r][c]);
        }
    }
    ```
C)
    ```java
    for (int c = grid[0].length - 2; c >= 0; c--) {
        for (int r = 0; r < grid.length; r++) {
            result.add(grid[r][c]);
        }
    }
    ```
D)
    ```java
    for (int r = grid.length - 1; r >= 0; r--) {
        for (int c = grid[0].length - 1; c >= 0; c--) {
            result.add(grid[r][c]);
        }
    }
    ```

::: {custom-style="QuestionBody"}
**Question 33.** Consider the following code segment.
:::
```java
int[] data = {5, 10, 15, 20, 25, 30, 35};
for (int i = 0; i < data.length; i += 3) {
    data[i] = data[i] * 2;
}
```
What are the contents of **data** after executing this code segment?

A) {10, 10, 15, 40, 25, 30, 70}
B) {5, 20, 15, 20, 50, 30, 35}
C) {10, 10, 30, 20, 50, 30, 70}
D) {10, 10, 15, 40, 25, 30, 35}

::: {custom-style="QuestionBody"}
**Question 34.** Consider the following code segment.
:::
```java
ArrayList<Integer> oldList = new ArrayList<Integer>();
oldList.add(1);
oldList.add(2);
oldList.add(3);
ArrayList<Integer> newList = new ArrayList<Integer>();
/* missing code */
```
After executing this code segment, `newList` should contain
`[1, 2, 3, 3, 2, 1]`.
Which of the following can replace `/* missing code */` to produce this result?

A)
    ```java
    for (Integer n : oldList) {
        newList.add(n);
        newList.add(n);
    }
    ```
B)
    ```java
    for (Integer n : oldList) {
        newList.add(0, n);
        newList.add(n);
    }
    ```
C)
    ```java
    for (int i = 0; i < oldList.size(); i++) {
        newList.add(oldList.get(0));
        newList.add(oldList.get(i));
    }
    ```
D)
    ```java
    for (int i = 0; i < oldList.size(); i++) {
        newList.add(i, oldList.get(i));
        newList.add(oldList.get(i));
    }
    ```
E)
    ```java
    for (int i = oldList.size() - 1; i >= 0; i--) {
        newList.add(oldList.get(i));
        newList.add(0, oldList.get(i));
    }
    ```

::: {custom-style="QuestionBody"}
**Question 35.** Consider the following method.
:::
```java
public static boolean ordered(ArrayList<Integer> vals) {
    int last = Integer.MIN_VALUE;
    for (int i = 0; i < vals.size(); i++) {
        int cur = vals.get(i);
        if (cur < last) {
            return false;
        }
        last = cur;
    }
    return true;
}
```
Which of the following best describes the behavior of the method?

A) It returns true if the list is sorted in non‑decreasing order (least to greatest, allowing duplicates) and false otherwise.
B) It returns true if the list is sorted in non‑increasing order (greatest to least) and false otherwise.
C) It returns false if the list is sorted in non‑decreasing order and true otherwise.
D) It always returns true regardless of the order of the elements.

::: {custom-style="QuestionBody"}
**Question 36.** Consider the following code segment.
:::
```java
int[][] matrix = new int[3][4];
for (int row = 0; row < matrix.length; row++) {
    for (int col = 0; col < matrix[0].length; col++) {
        if (row == col) {
            matrix[row][col] = row;
        }
    }
}
```
Which of the following represents the contents of **matrix** after executing this code segment?

A) {{0, 0, 0, 0},
 {0, 1, 0, 0},
 {0, 0, 2, 0}}
B) {{0, 0, 0},
 {0, 1, 0},
 {0, 0, 2},
 {0, 0, 0}}
C) {{0, 0, 0, 0},
 {0, 0, 1, 0},
 {0, 0, 0, 2}}
D) {{0, 0, 0, 0},
 {0, 0, 0, 0},
 {0, 0, 0, 0}}

::: {custom-style="QuestionBody"}
**Question 37.** Examine the following method. When an element of the two-dimensional array matrix is accessed, the first index specifies the row and the second index specifies the column.
:::
```java
public static int tally(int[][] matrix, int col, int key)
{
   int count = 0;
   for (int i = 0; i < matrix.length; i++)
   {
      if (matrix[i][col] == key)
      {
         count++;
      }
   }
   return count;
}
```
Which of the following best describes the value returned by the method?

A) The index of a column that contains at least one element equal to key
B) The number of times key appears in row col of matrix
C) The total number of occurrences of key anywhere in the matrix
D) The number of times key appears in column col of matrix

::: {custom-style="QuestionBody"}
**Question 38.** A text file named `data.txt` contains the following lines (there are no spaces in the file):
:::
```
apple_red
banana_yellow
grape_purple
orange_orange
```
The following code segment creates a `Scanner` object `sc` that reads from the file.
```java
File file = new File("data.txt");
Scanner sc = new Scanner(file);
ArrayList<String> firstList = new ArrayList<>();
ArrayList<String> secondList = new ArrayList<>();
/* missing code */
```
The intended result is that `firstList` contains `["apple", "banana", "grape", "orange"]` and `secondList` contains `["red", "yellow", "purple", "orange"]`.
Which of the following code fragments can replace `/* missing code */` so that the program works as intended?

A)
    ```java
    while (sc.hasNext()) {
        firstList.add(sc.next());
        secondList.add(sc.next());
    }
    ```
B)
    ```java
    while (sc.hasNextLine()) {
        String line = sc.nextLine();
        String[] parts = line.split("_");
        firstList.add(parts[0]);
    }
    while (sc.hasNextLine()) {
        String line = sc.nextLine();
        String[] parts = line.split("_");
        secondList.add(parts[1]);
    }
    ```
C)
    ```java
    while (sc.hasNextLine()) {
        String[] parts = sc.nextLine().split("_");
        firstList.add(parts[0]);
        firstList.add(parts[1]);
    }
    while (sc.hasNextLine()) {
        String[] parts = sc.nextLine().split("_");
        secondList.add(parts[0]);
        secondList.add(parts[1]);
    }
    ```
D)
    ```java
    while (sc.hasNextLine()) {
        String[] parts = sc.nextLine().split("_");
        firstList.add(parts[0]);
        secondList.add(parts[1]);
    }
    ```

::: {custom-style="QuestionBody"}
**Question 39.** The following method implements a recursive binary search algorithm.
:::
```java
public static int binarySearch(int[] arr, int low, int high, int key) {
    if (low <= high) {
        int mid = (low + high) / 2;
        if (key < arr[mid]) {
            return binarySearch(arr, low, mid - 1, key);
        } else if (key > arr[mid]) {
            return binarySearch(arr, mid + 1, high, key);
        } else {
            return mid;
        }
    }
    return -1;
}
```
The following code segment appears in a method in the same class as binarySearch.
```java
int[] data = {5, 7, 7, 7, 12, 15, 20, 20, 25};
int answer = binarySearch(data, 0, data.length - 1, 20);
```
What is the value stored in **answer** after executing the code segment?

A) -1
B) 6
C) 7
D) 20

::: {custom-style="QuestionBody"}
**Question 40.** The following method correctly implements the insertion‑sort algorithm. It sorts the elements of **numList** from least to greatest.
:::
```java
public static void insertionSort(ArrayList<Integer> numList) {
    for (int j = 1; j < numList.size(); j++) {
        int temp = numList.get(j);
        int k = j;
        while (k > 0 && temp < numList.get(k - 1)) {
            numList.set(k, numList.get(k - 1));
            k--;
        }
        numList.set(k, temp);
        /* end of outer loop */
    }
}
```
Assume **insertionSort** is called with an **ArrayList** that has been initialized with these Integer objects:

`[45, 25, 35, 15, 55, 5]`

What will the contents of **numList** be after **two** passes of the outer loop (i.e., when `j == 2` at the point indicated by `/* end of outer loop */`)?

A) [25, 35, 45, 15, 55, 5]
B) [15, 25, 35, 45, 55, 5]
C) [45, 35, 25, 15, 55, 5]
D) [45, 25, 35, 15, 55, 5]

::: {custom-style="QuestionBody"}
**Question 41.** Consider the following code segment.
:::
```java
String[] planets = {"Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn"};
int index = -1;
for (int p = 0; p < planets.length; p++) {
    for (int q = p + 1; q < planets.length; q++) {
        if (planets[p].length() == planets[q].length()) {
            index = q;
        }
    }
}
```
What is the value of **index** after executing this code segment?

A) -1
B) 4
C) 5
D) 2

::: {custom-style="QuestionBody"}
**Question 42.** Consider the following code segment.
:::
```java
int i = 4;
int total = 0;
while (i > 0) {
    total += i;
    i--;
}
```
Which of the following code segments assigns the same value to **total** as the preceding code segment?

A)
    ```java
    int total = 0;
    for (int i = 5; i >= 1; i--) {
        total += i;
    }
    ```
B)
    ```java
    int total = 0;
    for (int i = 4; i > 1; i--) {
        total += i;
    }
    ```
C)
    ```java
    int total = 0;
    int i = 4;
    while (i > 0) {
        i--;
        total += i;
    }
    ```
D)
    ```java
    int total = 0;
    for (int i = 4; i >= 1; i--) {
        total += i;
    }
    ```

## Answers

1. E
2. B
3. D
4. D
5. E
6. A
7. B
8. D
9. A
10. A
11. A
12. D
13. D
14. A
15. A
16. D
17. D
18. A
19. E
20. B
21. A
22. D
23. B
24. A
25. C
26. A
27. A
28. B
29. A
30. A
31. A
32. A
33. A
34. E
35. A
36. A
37. D
38. D
39. B
40. A
41. D
42. D
