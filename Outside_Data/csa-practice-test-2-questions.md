# AP Computer Science A Practice Exam #2 (Questions 1-9)

Transcribed from `csa-practice-test-2.pdf`, pages 1-10 (booklet pages 1-10 of 50).

---

## Question 1

Consider the following code segment.

```java
int w = 50;
int x = 20;
int y = x;
x = w;
w = x + y;
System.out.println(w + " " + x);
```

What is printed as a result of executing the code segment?

- (A) `40 20`
- (B) `70 20`
- (C) `70 50`
- (D) `100 50`

---

## Question 2

A student is developing a program that will store information about players of a video game, including each player's score and amount of time remaining, in seconds. Which of the following is the most appropriate design?

- (A) An `Information` class with `Player` instance variables for the score and the amount of time remaining
- (B) A `Player` class with `int` instance variables for the score and the amount of time remaining
- (C) A `Player` class, a `Score` class, and a `RemainingTime` class, each with an `int` instance variable
- (D) A `Score` class and a `RemainingTime` class, each with a `Player` instance variable

---

## Question 3

Which of the following expressions evaluates to `3.5`?

- (A) `(double) 2 / 4 + 3`
- (B) `(double) 2 / (4 + 3)`
- (C) `(double) (2 / 4) + 3`
- (D) `(double) (2 / 4 + 3)`

---

## Question 4

Consider the following code segment.

```java
String oldStr = "ABCDEF";
String newStr = /* missing code */;
System.out.println(newStr);
```

Which of the following expressions can be used to replace `/* missing code */` so that the code segment prints the string `"BCEF"`?

- (A) `oldStr.substring(1, 2) + oldStr.substring(4)`
- (B) `oldStr.substring(1, 2) + oldStr.substring(5)`
- (C) `oldStr.substring(1, 3) + oldStr.substring(4)`
- (D) `oldStr.substring(1, 3) + oldStr.substring(5)`

---

## Question 5

Consider the following partial class definition.

```java
public class SomeClass
{
   private int myA;

   // Constructor(s) not shown

   public int getA()
   {
      return myA;
   }
}
```

Assume that `SomeClass` object `obj` has been properly declared and instantiated in a class other than `SomeClass`. Which of the following statements will compile without error?

- (A) `int x = obj.getA();`
- (B) `int x = obj.myA;`
- (C) `int x = SomeClass.getA();`
- (D) `int x = getA(obj);`

---

## Question 6

Which of the following methods can be added to `SomeClass` to allow other classes to modify the value of `myA`?

- (A)
  ```java
  public void setA(int newA)
  {
     myA = newA;
  }
  ```
- (B)
  ```java
  private void setA(int newA)
  {
     myA = newA;
  }
  ```
- (C)
  ```java
  public void setA(int newA)
  {
     newA = myA;
  }
  ```
- (D)
  ```java
  private void setA(int newA)
  {
     newA = myA;
  }
  ```

---

## Question 7

The following categories are used by some researchers to categorize regions as urban, suburban, or rural based on population density.

- An urban region is a region with more than 3,000 people per square mile.
- A suburban region is a region with between 1,000 and 3,000 people, inclusive, per square mile.
- A rural region is a region with fewer than 1,000 people per square mile.

Consider the following method, which is intended to categorize a region as urban, suburban, or rural based on the population density of the region.

```java
public static String getCategory(int density)
{
   /* missing code */
}
```

Which of the following code segments can replace `/* missing code */` so the `getCategory` method works as intended?

- (A)
  ```java
  String category;
  if (density > 3000)
  {
     category = "urban";
  }
  else if (density >= 1000)
  {
     category = "suburban";
  }
  else
  {
     category = "rural";
  }
  return category;
  ```
- (B)
  ```java
  String category;
  if (density > 3000)
  {
     category = "urban";
  }
  else if (density <= 3000)
  {
     category = "suburban";
  }
  else
  {
     category = "rural";
  }
  return category;
  ```
- (C)
  ```java
  String category = "rural";
  if (density > 3000)
  {
     category = "urban";
  }
  if (density >= 1000)
  {
     category = "suburban";
  }
  return category;
  ```
- (D)
  ```java
  String category = "urban";
  if (density < 1000)
  {
     category = "rural";
  }
  if (density >= 1000)
  {
     category = "suburban";
  }
  return category;
  ```

---

## Question 8

Consider the following class definition.

```java
public class Student
{
   private String myName;
   private int myAge;

   public Student()
   { /* implementation not shown */ }

   public Student(String name)
   { /* implementation not shown */ }

   public Student(String name, int age)
   { /* implementation not shown */ }

   // There are no other constructors.
}
```

Which of the following statements will cause a compilation error?

- (A) `Student a = new Student();`
- (B) `Student b = new Student("Juan");`
- (C) `Student c = new Student("Juan", 15);`
- (D) `Student d = new Student("Juan", "15");`

---

## Question 9

Consider the following method, which is intended to return `true` if the distance between `d1` and `d2` is no more than `tolerance` units, and to return `false` otherwise.

For example, `almostEqual(7.9, 7.8, 0.15)` should return `true` because the distance between `7.9` and `7.8` is less than `0.15`, and `almostEqual(54.124, 54.126, 0.001)` should return `false` because the distance between `54.124` and `54.126` is greater than `0.001`.

```java
public static boolean almostEqual(double d1, double d2,
                                  double tolerance)
{
   /* missing code */;
}
```

Which of the following can be used to replace `/* missing code */` so that the method will work as intended?

- (A) `return (d1 - d2) <= tolerance;`
- (B) `return ((d1 + d2) / 2) <= tolerance;`
- (C) `return Math.abs(d1 - d2) <= tolerance;`
- (D) `return Math.abs((d1 + d2) / 2) <= tolerance;`

---

## Question 10

Consider the following code segment.

```java
int a = 3 + 2 * 3;
int b = 4 + 3 / 2;
int c = 7 % 4 + 3;
double x = a + b + c;
```

What is the value of `x` after the code segment is executed?

- (A) `14.0`
- (B) `18.0`
- (C) `20.0`
- (D) `20.5`

---

## Question 11

In the following code segment, `num` is a properly declared and initialized `int` variable.

```java
if (num > 0)
{
   if (num % 2 == 0)
   {
      System.out.println("X");
   }
   else
   {
      System.out.println("Y");
   }
}
```

Which of the following best describes the behavior of the code segment?

- (A) When `num` is a negative odd integer, `"X"` is printed; otherwise, `"Y"` is printed.
- (B) When `num` is a positive even integer, `"X"` is printed; otherwise, `"Y"` is printed.
- (C) When `num` is a positive even integer, `"X"` is printed; when `num` is a positive odd integer, `"Y"` is printed; otherwise, nothing is printed.
- (D) When `num` is a positive odd integer, `"X"` is printed; when `num` is a positive even integer, `"Y"` is printed; otherwise, nothing is printed.

---

## Question 12

In the following code segment, `num` is a properly declared and initialized `int` variable.

```java
boolean b1 = true;
if (num > 0)
{
   if (num >= 100)
   {
      b1 = false;
   }
}
else
{
   if (num >= -100)
   {
      b1 = false;
   }
}
```

Which of the following statements always assigns the same value to `b2` as the code segment assigns to `b1`?

- (A) `boolean b2 = (num > -100) && (num < 100);`
- (B) `boolean b2 = (num > -100) || (num < 100);`
- (C) `boolean b2 = (num < -100) && (num > 0 || num < 100);`
- (D) `boolean b2 = (num < -100) || (num > 0 && num < 100);`

---

## Question 13

A programmer is developing Java code that can be used to analyze a data file. In which of the following situations would it be best for the programmer to publish the code as open source?

- (A) When the programmer wants to enable other programmers to modify the code for inclusion in their programs
- (B) When the programmer wants to ensure that attributes are encapsulated
- (C) When the programmer wants to prevent others from sharing the code
- (D) When the programmer wants to require other programmers to purchase the code

---

## Question 14

Assume that `a`, `b`, and `c` are `boolean` variables that have been properly declared and initialized. Which of the following boolean expressions is equivalent to `!(a && b) || c`?

- (A) `a || b || c`
- (B) `!a && !b || c`
- (C) `!a && !b && c`
- (D) `!a || !b || c`

---

## Question 15

Which of the following code segments produces the output `"987654321"`?

- (A)
  ```java
  int num = 10;
  while (num > 0)
  {
     System.out.print(num);
     num--;
  }
  ```
- (B)
  ```java
  int num = 10;
  while (num > 1)
  {
     System.out.print(num);
     num--;
  }
  ```
- (C)
  ```java
  int num = 10;
  while (num > 1)
  {
     num--;
     System.out.print(num);
  }
  ```
- (D)
  ```java
  int num = 10;
  while (num >= 1)
  {
     num--;
     System.out.print(num);
  }
  ```

---

## Question 16

Consider the following class definitions.

```java
public class Point
{
   private double num1;
   private double num2;

   public Point(int n1, int n2)
   {
      num1 = n1;
      num2 = n2;
   }
}

public class PointChecker
{
   public static boolean checkPoint(Point p)
   {
      return p.num1 > 0.0 && p.num2 > 0.0;
   }
}
```

Which of the following best explains why an error occurs when the classes are compiled?

- (A) The `Point` constructor should have return type `void`.
- (B) In the `Point` constructor, `int` values cannot be assigned to `double` variables.
- (C) The `checkPoint` method cannot have a parameter of type `Point`.
- (D) In the `checkPoint` method, the instance variables `num1` and `num2` cannot be accessed from a class other than `Point`.

---

## Question 17

Consider the following methods.

```java
/** Preconditions: a > 0 and b > 0 */
public static int methodX(int a, int b)
{
   int loopCount = 0;
   for (int i = 0; i < a / b; i++)
   {
      loopCount++;
   }
   return loopCount;
}

/** Preconditions: a > 0 and b > 0 */
public static int methodY(int a, int b)
{
   int loopCount = 0;
   int i = 0;
   while (i < a)
   {
      loopCount++;
      i += b;
   }
   return loopCount;
}
```

Which of the following best describes the conditions under which `methodX` and `methodY` return the same value?

- (A) When `a` and `b` are both even
- (B) When `a` and `b` are both odd
- (C) When `a % b` is equal to zero
- (D) When `a % b` is equal to one

---

## Question 18

Consider the following code segment.

```java
int x = 1;
while (x < 10)
{
   if (x % 2 == 0)
   {
      System.out.print(x + " ");
   }
   /* missing code */
}
```

Which of the following replacements for `/* missing code */` will cause the code segment to print nothing?

- (A) `x++;`
- (B) `x += 2;`
- (C) `x += 3;`
- (D) `x *= 2;`

---

## Question 19

Consider the following code segment.

```java
ArrayList<Integer> numList = new ArrayList<Integer>();

numList.add(3);
numList.add(2);
numList.add(1);
numList.add(1, 0);
numList.set(0, 2);
```

Which of the following represents the contents of `numList` after executing the code segment?

- (A) `[1, 3, 0, 1]`
- (B) `[2, 0, 2, 1]`
- (C) `[2, 3, 2, 1]`
- (D) `[3, 0, 0, 1]`

---

## Question 20

Consider the following code segment.

```java
for (int outer = 1; outer <= 4; outer++)
{
   for (int inner = outer; inner <= 4; inner++)
   {
      System.out.print(inner + " ");
   }
   System.out.println();
}
```

What is printed as a result of executing the code segment?

- (A)
  ```text
  1 1 1 1
  2 2 2
  3 3
  4
  ```
- (B)
  ```text
  1 1 1 1
  2 2 2 2
  3 3 3 3
  4 4 4 4
  ```
- (C)
  ```text
  1 2 3 4
  2 3 4
  3 4
  4
  ```
- (D)
  ```text
  1 2 3 4
  1 2 3 4
  1 2 3 4
  1 2 3 4
  ```

---

## Question 21

Which of the following actions is most likely to improve the system reliability of a Java program?

- (A) Designing the program to minimize the use of procedural abstraction
- (B) Testing the program with conditions that represent a wide variety of ways the program will be used
- (C) Using default constructors to set the initial state of the objects used in the program
- (D) Using `public` visibility for the attributes and behaviors of the classes used in the program

---

## Question 22

Consider the following two code segments. Assume that the `int` variables `m` and `n` have been properly declared and initialized and are both greater than `0`.

**Code Segment 1**

```java
for (int i = 0; i < m * n; i++)
{
   System.out.print("*");
}
```

**Code Segment 2**

```java
for (int j = 1; j <= m; j++)
{
   for (int k = 1; k < n; k++)
   {
      System.out.print("@");
   }
}
```

Assume that the initial values of `m` and `n` are the same in Code Segment 1 as they are in Code Segment 2. Which of the following correctly compares the number of times that `"*"` and `"@"` are printed when each code segment is executed?

- (A) `"*"` is printed `m` fewer times than `"@"`.
- (B) `"*"` is printed `n` fewer times than `"@"`.
- (C) `"*"` is printed `m` more times than `"@"`.
- (D) `"*"` is printed `n` more times than `"@"`.

---

## Question 23

Consider the following method.

```java
/** Precondition: num > 0 */
public static int doWhat(int num)
{
   int var = 0;
   for (int loop = 1; loop <= num; loop = loop + 2)
   {
      var += loop;
   }
   return var;
}
```

Which of the following best describes the value, if any, returned from a call to `doWhat`?

- (A) The sum of all integers between `1` and `num`, inclusive
- (B) The sum of all even integers between `1` and `num`, inclusive
- (C) The sum of all odd integers between `1` and `num`, inclusive
- (D) No value is returned because of an infinite loop.

---

## Question 24

Consider an `int` array named `nums`, which has been properly declared and initialized with one or more values. Which of the following code segments counts the number of negative values found in `nums` and stores the count in `counter`?

- (A)
  ```java
  int counter = 0;
  int i = 0;
  while (i < nums.length)
  {
     if (nums[i] < 0)
     {
        counter++;
        i++;
     }
  }
  ```
- (B)
  ```java
  int counter = 0;
  int i = 0;
  while (i <= nums.length - 1)
  {
     if (nums[i] < 0)
     {
        counter++;
     }
     i++;
  }
  ```
- (C)
  ```java
  int counter = 0;
  for (int i = 1; i < nums.length; i++)
  {
     if (nums[i] < 0)
     {
        counter++;
     }
  }
  ```
- (D)
  ```java
  int counter = 0;
  for (int i : nums)
  {
     if (nums[i] < 0)
     {
        counter++;
     }
  }
  ```

---

## Question 25

Consider the following class definition.

```java
public class SomeClass
{
   private int x = 0;
   private static int y = 0;

   public SomeClass(int pX)
   {
      x = pX;
      y++;
   }

   public void incrementY()
   {
      y++;
   }

   public void incrementY(int inc)
   {
      y += inc;
   }

   public int getY()
   {
      return y;
   }
}
```

The following code segment appears in a class other than `SomeClass`. Assume that no other `SomeClass` objects have been created.

```java
SomeClass first = new SomeClass(10);
SomeClass second = new SomeClass(20);
SomeClass third = new SomeClass(30);
first.incrementY();
second.incrementY(10);
System.out.println(third.getY());
```

What is printed as a result of executing the code segment?

- (A) `0`
- (B) `1`
- (C) `11`
- (D) `14`

---

## Question 26

Consider the following code segment.

```java
String one = "xyz";
String two = one.substring(2) + one.substring(1)
           + one.substring(0);
int pos = two.indexOf("x");
```

What is the value of `pos` after executing the code segment?

- (A) `-1`
- (B) `2`
- (C) `3`
- (D) `5`

---

## Question 27

Consider the following method.

```java
public static int mystery(int[] arr)
{
   int x = 0;
   for (int k = 0; k < arr.length; k = k + 2)
   {
      x = x + arr[k];
   }
   return x;
}
```

Assume that the array `nums` has been declared and initialized as follows.

```java
int[] nums = {3, 6, 1, 0, 1, 4, 2};
```

What value is returned as a result of the call `mystery(nums)`?

- (A) `5`
- (B) `7`
- (C) `10`
- (D) `17`

---

## Question 28

Consider the following method. When an element of the two-dimensional array `values` is accessed, the first index is used to specify the row and the second index is used to specify the column.

```java
public static int calculate(int[][] values, int x)
{
   int result = 0;
   for (int y = 0; y < values[0].length; y++)
   {
      if (values[x][y] > 0)
      {
         result += values[x][y];
      }
   }
   return result;
}
```

Which of the following best describes the value returned by the `calculate` method?

- (A) A count of the positive elements in column `x` of `values`
- (B) A count of the positive elements in row `x` of `values`
- (C) The sum of the positive elements in column `x` of `values`
- (D) The sum of the positive elements in row `x` of `values`

---

## Question 29

A text file named `bookInfo.txt` has the following contents. Assume that the file contains no spaces.

```text
Persuasion,Austen_1818
Tarzan,Burroughs_1912
Frankenstein,Shelley_1818
Dracula,Stoker_1897
```

In the following code segment, a valid `Scanner` object named `sc` is created to read from the text file.

```java
File bookData = new File("bookInfo.txt");
Scanner sc = new Scanner(bookData);

ArrayList<String> aList = new ArrayList<String>();
ArrayList<String> bList = new ArrayList<String>();
ArrayList<String> cList = new ArrayList<String>();

while (sc.hasNext())
{
   String temp = sc.next();
   int j = temp.indexOf(",");
   int k = temp.indexOf("_");

   aList.add(temp.substring(0, j));
   bList.add(temp.substring(j + 1, k));
   cList.add(temp.substring(k + 1));
}

System.out.println(cList.get(1));
```

What is printed as a result of executing the code segment?

- (A) `1818`
- (B) `1912`
- (C) `Austen`
- (D) `Burroughs`

---

## Question 30

A programmer wants to investigate whether players of an online game who are younger than 30 years old tend to spend more time playing the game than players who are 30 years old or older. Each player of the game must create an account with a unique username. The programmer has the following data sets available.

- Data set 1 contains an entry for each player. Each entry contains the player's birth date (month, day, and year) and username.
- Data set 2 contains an entry for each player. Each entry contains the player's username and the average number of hours the player spends playing the game each day.
- Data set 3 contains an entry for each year since the game was released. Each entry includes the number of players who were born in that year.
- Data set 4 contains an entry for each year since the game was released. Each entry includes a list of usernames of the players who created an account in that year.

Which two data sets can be combined and analyzed to determine the desired information?

- (A) Data sets 1 and 2
- (B) Data sets 1 and 3
- (C) Data sets 2 and 3
- (D) Data sets 2 and 4

---

## Question 31

Consider the following method.

```java
public static String mystery(String input)
{
   String output = "";
   for (int k = 1; k < input.length(); k = k + 2)
   {
      output += input.substring(k, k + 1);
   }
   return output;
}
```

What is returned as a result of the call `mystery("computer")`?

- (A) `"computer"`
- (B) `"cmue"`
- (C) `"optr"`
- (D) `"ompute"`

---

## Question 32

Consider the following class definition.

```java
public class Item
{
   private double price;

   public Item(double price)
   {
      price = price;
   }

   public double getPrice()
   {
      return price;
   }

   /* There are no other instance variables,
      methods, or constructors. */
}
```

The following code segment appears in a class other than `Item`. The code segment is intended to print the value `2.0` but does not work as intended.

```java
Item pen = new Item(2.0);
System.out.println(pen.getPrice());
```

Which of the following best explains why the code segment does not work as intended?

- (A) The code segment does not compile because the call to the `Item` constructor uses incorrect syntax.
- (B) The `Item` class does not compile because the `getPrice` method does not have access to a variable named `price`.
- (C) The instance variable `price` is not assigned a value in the `Item` constructor because the constructor does not return a value.
- (D) The instance variable `price` is not assigned a value in the `Item` constructor because the constructor assigns a value to the parameter instead of the instance variable.

---

## Question 33

Consider the following statement.

```java
int b = 5 + (int) (Math.random() * 5);
```

What are the possible values that can be assigned to `b` as a result of executing the statement?

- (A) `5, 6, 7, 8, or 9`
- (B) `5, 6, 7, 8, 9, or 10`
- (C) `6, 7, 8, or 9`
- (D) `6, 7, 8, 9, or 10`

---

## Question 34

Consider the following class definition.

```java
public class StudentInfo
{
   private String major;
   private int age;

   public String getMajor()
   { return major; }

   public int getAge()
   { return age; }

   /* There may be instance variables, constructors,
      and methods that are not shown. */
}
```

The following instance variable and method appear in another class. The `averageAgeInMajor` method is intended to return the average age of the `StudentInfo` objects in `studentList` whose major is equal to `theMajor`, or `-1` if there are no `StudentInfo` objects in `studentList` with a major that is equal to `theMajor`.

```java
private ArrayList<StudentInfo> studentList;

public double averageAgeInMajor(String theMajor)
{
   double sum = 0.0;
   int count = 0;

   for (StudentInfo k : studentList)
   {
      /* missing code */
   }
   if (count > 0)
   {
      return sum / count;
   }
   else
   {
      return -1.0;
   }
}
```

Which of the following can be used to replace `/* missing code */` so that `averageAgeInMajor` will work as intended?

- (A)
  ```java
  if (theMajor.equals(k.getMajor()))
  {
     count++;
  }
  sum += k.getAge();
  ```
- (B)
  ```java
  if (theMajor.equals(k.getMajor()))
  {
     sum += k.getAge();
     count++;
  }
  ```
- (C)
  ```java
  if (theMajor.equals(k.getMajor()))
  {
     sum = k.getAge();
     count++;
  }
  ```
- (D)
  ```java
  if (theMajor.equals(k.getMajor()))
  {
     sum += k.getAge();
  }
  count++;
  ```

---

## Question 35

Consider the following code segment.

```java
int[][] values = {{1, 2, 3},
                  {4, 5, 6}};
int x = 0;

for (int j = 0; j < values.length; j++)
{
   for (int k = 0; k < values[0].length; k++)
   {
      if (k == 0)
      {
         values[j][k] *= 2;
      }
      x += values[j][k];
   }
}
```

What is the value of `x` after executing the code segment?

- (A) `7`
- (B) `17`
- (C) `21`
- (D) `26`

---

## Question 36

Consider the following `mode` method, which is intended to return the most frequently occurring value (mode) in its `int[]` parameter `arr`. If there is more than one mode, the method is intended to return the first mode that appears in the array. For example, if the parameter has the contents `{6, 5, 1, 5, 2, 6, 5}`, then the method is intended to return `5`. As another example, if the parameter has the contents `{3, 8, 4, 8, 4, 6, 5}`, then the method is intended to return `8`.

```java
/** Precondition: arr.length >= 1 */
public static int mode(int[] arr)
{
   int modeCount = 1;
   int mode = arr[0];

   for (int j = 0; j < arr.length; j++)
   {
      int valCount = 0;
      for (int k = 0; k < arr.length; k++)
      {
         if (/* missing condition 1 */)
         {
            valCount++;
         }
      }
      if (/* missing condition 2 */)
      {
         modeCount = valCount;
         mode = arr[j];
      }
   }
   return mode;
}
```

Which of the following can replace `/* missing condition 1 */` and `/* missing condition 2 */` so the method works as intended?

- (A)

  | `/* missing condition 1 */` | `/* missing condition 2 */` |
  | --- | --- |
  | `arr[j] == arr[k]` | `valCount > modeCount` |

- (B)

  | `/* missing condition 1 */` | `/* missing condition 2 */` |
  | --- | --- |
  | `arr[j] == arr[k]` | `modeCount > valCount` |

- (C)

  | `/* missing condition 1 */` | `/* missing condition 2 */` |
  | --- | --- |
  | `arr[j] != arr[k]` | `valCount > modeCount` |

- (D)

  | `/* missing condition 1 */` | `/* missing condition 2 */` |
  | --- | --- |
  | `arr[j] != arr[k]` | `modeCount > valCount` |

---

## Question 37

Consider the following recursive method.

```java
/** Precondition: num >= 0 */
public static int what(int num)
{
   if (num < 10)
   {
      return 1;
   }
   else
   {
      return 1 + what(num / 10);
   }
}
```

Assume that the `int` variable `val` has been declared and initialized with a value that satisfies the precondition of the method. Which of the following best describes the value, if any, returned by the call `what(val)`?

- (A) The number of digits in `val`
- (B) The sum of the digits in `val`
- (C) The value `1`
- (D) Nothing is returned. A run-time error occurs because of infinite recursion.

---

## Question 38

Consider the following class definition.

```java
public class Book
{
   private int pages;

   public int getPages()
   {
      return pages;
   }

   /* There may be instance variables, constructors,
      and methods that are not shown. */
}
```

The following method appears in a class other than `Book`. The method is intended to return the greatest number of pages found in any `Book` object in the `ArrayList` parameter `bookList`.

```java
public static int findMaxPages(ArrayList<Book> bookList)
{
   int maxPages = Integer.MIN_VALUE;

   for (Book b : bookList)
   {
      /* missing code */
   }

   return maxPages;
}
```

Which of the following can replace `/* missing code */` so the method works as intended?

- (A)
  ```java
  if (b.getPages() < maxPages)
  {
     maxPages = b.getPages();
  }
  ```
- (B)
  ```java
  if (b.getPages() > maxPages)
  {
     maxPages = b.getPages();
  }
  ```
- (C)
  ```java
  if (bookList.get(b).getPages() < maxPages)
  {
     maxPages = bookList.get(b).getPages();
  }
  ```
- (D)
  ```java
  if (bookList.get(b).getPages() > maxPages)
  {
     maxPages = bookList.get(b).getPages();
  }
  ```

---

## Question 39

Consider the following method.

```java
/**
 * Precondition: matrix has the same number of
 *               rows and columns.
 */
public static int[] operation(int[][] matrix, int r, int c)
{
   int[] result = new int[matrix.length];

   for (int j = 0; j < matrix.length; j++)
   {
      result[j] = matrix[r][j] * matrix[j][c];
   }
   return result;
}
```

The following code segment appears in another method in the same class.

```java
int[][] mat = {{3, 1, 2, 4},
               {2, 0, 3, 1},
               {1, 2, 3, 3},
               {1, 1, 1, 1}};

int[] arr = operation(mat, 1, 2);
```

Which of the following represents the contents of `arr` after executing the code segment?

- (A) `{1, 0, 6, 3}`
- (B) `{1, 9, 0, 4}`
- (C) `{4, 0, 9, 1}`
- (D) `{4, 3, 6, 2}`

---

## Question 40

Consider the following method. This method correctly sorts the elements of array `data` into increasing order.

```java
public static void sort(int[] data)
{
   for (int j = 0; j < data.length - 1; j++)
   {
      int m = j;
      for (int k = j + 1; k < data.length; k++)
      {
         if (data[k] < data[m])   /* Compare values */
         {
            m = k;
         }
      }
      int temp = data[m];         /* Assign to temp */
      data[m] = data[j];
      data[j] = temp;
      /* End of outer loop */
   }
}
```

Assume that `sort` is called with the array `{6, 3, 2, 5, 4, 1}`. Which of the following represents the contents of `data` after three passes of the outer loop (i.e., when `j == 2` at the point indicated by `/* End of outer loop */`)?

- (A) `{1, 2, 3, 4, 5, 6}`
- (B) `{1, 2, 3, 5, 4, 6}`
- (C) `{1, 2, 3, 6, 5, 4}`
- (D) `{1, 3, 2, 5, 4, 6}`

---

## Question 41

Assume that `sort` is called with the array `{1, 2, 3, 4, 5, 6}`. How many times will the expression indicated by `/* Compare values */` and the statement indicated by `/* Assign to temp */` execute?

- (A)

  | `/* Compare values */` | `/* Assign to temp */` |
  | --- | --- |
  | 15 times | 5 times |

- (B)

  | `/* Compare values */` | `/* Assign to temp */` |
  | --- | --- |
  | 15 times | 6 times |

- (C)

  | `/* Compare values */` | `/* Assign to temp */` |
  | --- | --- |
  | 21 times | 5 times |

- (D)

  | `/* Compare values */` | `/* Assign to temp */` |
  | --- | --- |
  | 21 times | 6 times |

---

## Question 42

Consider the following recursive method.

```java
public static void stars(int num)
{
   if (num == 1)
   {
      return;
   }

   stars(num - 1);

   for (int i = 0; i < num; i++)
   {
      System.out.print("*");
   }
   System.out.println();
}
```

What is printed as a result of the method call `stars(5)`?

- (A)
  ```text
  *****
  ****
  ***
  **
  *
  ```
- (B)
  ```text
  *****
  ****
  ***
  **
  ```
- (C)
  ```text
  *
  **
  ***
  ****
  *****
  ```
- (D)
  ```text
  **
  ***
  ****
  *****
  ```
