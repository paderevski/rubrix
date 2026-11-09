# AP Computer Science A Practice Exam #1 (Questions 1-10)

Transcribed from `csa-practice-test-1.pdf`, pages 1-5 (exam pages 11-15).

---

## Question 1

Consider the following code segment.

```java
int x = 20;
int y = 10;
int temp = x;
x = y;
y = temp * x;
System.out.println(x + " " + y);
```

What is printed as a result of executing this code segment?

- (A) `10 100`
- (B) `10 200`
- (C) `20 100`
- (D) `20 200`

---

## Question 2

In the following statement, `j` and `k` are properly declared and initialized `int` variables.

```java
boolean result = ((j > 0) && (k > 0)) || ((j < 0) && (k < 0));
```

Which of the following best describes the conditions in which `result` is assigned the value `true`?

- (A) When `j` and `k` are both equal to `0`
- (B) When `j` and `k` are both positive or both negative
- (C) When `j` is greater than `k`
- (D) When `j` is positive and `k` is negative or when `j` is negative and `k` is positive

---

## Question 3

Consider the following `Painting` class.

```java
public class Painting
{
   private int year;
   private String artist;
   private String title;

   public Painting(int y, String a)
   {
      year = y;
      artist = a;
      title = "Untitled";
   }

   // There are no other constructors.
}
```

Which of the following code segments, appearing in a class other than `Painting`, will correctly create an instance of a `Painting` object?

- (A) `Painting p = new Painting("Frida Kahlo", "Self Portrait", 1939);`
- (B) `Painting p = new Painting(1939, "Frida Kahlo", "Self Portrait");`
- (C) `Painting p = new Painting("Frida Kahlo", 1939);`
- (D) `Painting p = new Painting(1939, "Frida Kahlo");`

---

## Question 4

Consider the following code segment.

```java
int first = 5 + 10 * 2;
int second = first + first % 2;
```

What is the value of `second` after this code segment is executed?

- (A) `37`
- (B) `30`
- (C) `26`
- (D) `0`

---

## Question 5

Consider the following code segment.

```java
double cost = 25.0;
double discount = 0.0;

if (cost >= 100.0)
{
   discount = 10.0;
}
else if (cost >= 50.0)
{
   discount = 5.0;
}
else
{
   discount = 2.0;
}

System.out.println(cost - discount);
```

What is printed as a result of executing this code segment?

- (A) `25.0`
- (B) `23.0`
- (C) `20.0`
- (D) `15.0`

---

## Question 6

Consider the following code segment.

```java
for (int j = 10; j >= 0; j -= 2)   // Line 1
{                                   // Line 2
   System.out.print(j - 1);         // Line 3
}                                   // Line 4
```

This code segment is intended to produce only the following output. It does not work as intended.

```
97531
```

Which of the following changes can be made so that this code segment works as intended?

- (A) In line 1, changing the condition `j >= 0` to `j > 0`
- (B) In line 1, changing the condition `j = 10` to `j = 9`
- (C) In line 3, changing the condition `j - 1` to `j + 1`
- (D) In line 3, changing the condition `j - 1` to `j - 2`

---

## Question 7

Consider the following code segment.

```java
String original = "DEFGHI";
String result = /* missing code */;
System.out.println(result);
```

Which of the following expressions can be used to replace `/* missing code */` so that this code segment prints the string `"HIDE"`?

- (A) `original.substring(4) + original.substring(0, 1)`
- (B) `original.substring(4) + original.substring(0, 2)`
- (C) `original.substring(5) + original.substring(1, 2)`
- (D) `original.substring(5) + original.substring(1, 3)`

---

## Question 8

Consider the following code segment.

```java
double w = 2.0;
double x = 5.0;
double y = (int) w / x;
double z = (int) (w / x);

System.out.println(y + ", " + z);
```

What is printed as a result of executing this code segment?

- (A) `0.0, 0.0`
- (B) `0.0, 0.4`
- (C) `0.4, 0.0`
- (D) `0.4, 0.4`

---

## Question 9

Consider the following class definitions.

```java
public class Measurement
{
   private int feet;
   private int inches;

   public Measurement(int numFeet, int numInches)
   {
      feet = numFeet;
      inches = numInches;
   }

   private int toInches()
   {
      return feet * 12 + inches;
   }
}

public class Calculations
{
   public static boolean compare(Measurement m1, Measurement m2)
   {
      if (m1.toInches() >= m2.toInches())
      {
         return true;
      }
      return false;
   }
}
```

Which of the following best explains why an error occurs when the classes are compiled?

- (A) The `compare` method cannot have parameters of type `Measurement`.
- (B) The `compare` method should be declared as `void`.
- (C) The `toInches` method cannot be accessed from a class other than `Measurement`.
- (D) The variables `feet` and `inches` cannot be accessed from the method `toInches`.

---

## Question 10

A computer game should print different messages based on a player's score.

- `"Great!"` is printed for scores greater than `500`.
- `"Good!"` is printed for scores between `250` and `500`, inclusive.
- `"Try again."` is printed for scores less than `250`.

Assume that `score` is a properly declared and initialized `int` that represents a player's score.

Which of the following code segments will print the correct message for any valid value of `score`?

**(A)**

```java
String message = "";
if (score < 250)
{
   message = "Try again.";
}
else if (score <= 500)
{
   message = "Good!";
}
else
{
   message = "Great!";
}
System.out.println(message);
```

**(B)**

```java
String message = "";
if (score < 250)
{
   message = "Try again.";
}
if (score <= 500)
{
   message = "Good!";
}
else
{
   message = "Great!";
}
System.out.println(message);
```

**(C)**

```java
String message = "";
if (score >= 250)
{
   message = "Good!";
}
else if (score > 500)
{
   message = "Great!";
}
else
{
   message = "Try again.";
}
System.out.println(message);
```

**(D)**

```java
String message = "";
if (score > 500)
{
   message = "Great!";
}
else if (score <= 500)
{
   message = "Good!";
}
else
{
   message = "Try again.";
}
System.out.println(message);
```

---

## Questions 11 and 12

Questions 11 through 12 refer to the following class declaration.

```java
public class Pair
{
   private int x;
   private int y;

   public Pair(int x1, int y1)
   {
      x = x1;
      y = y1;
   }

   public int getX()
   {
      return x;
   }
}
```

### Question 11

Assume that `myPair` is a properly declared and initialized `Pair` object that appears in a class other than `Pair`. Which of the following code segments will compile without error?

- (A) `int val = myPair.getX();`
- (B) `int val = myPair.getX(x);`
- (C) `int val = myPair.x;`
- (D) `int val = myPair.x1;`

### Question 12

Which of the following methods can be added to the `Pair` class to allow other classes to modify the value of `y`?

- (A)
  ```java
  private int setY(int num)
  {
     return num;
  }
  ```
- (B)
  ```java
  public int setY(int num)
  {
     return num;
  }
  ```
- (C)
  ```java
  private void setY(int num)
  {
     y = num;
  }
  ```
- (D)
  ```java
  public void setY(int num)
  {
     y = num;
  }
  ```

---

## Question 13

Consider the following code segment.

```java
for (int outer = 0; outer < 5; outer++)
{
   for (int inner = 4; inner >= 0; inner--)
   {
      System.out.println("*");   // Line 5
   }
}
```

How many times will the statement in line 5 be executed as a result of running this code segment?

- (A) `20`
- (B) `24`
- (C) `25`
- (D) `30`

---

## Question 14

Assume that `isEven`, `isPositive`, and `isPrime` are `boolean` variables that have been properly declared and initialized. Which of the following expressions is equivalent to the expression `!(isEven && isPositive) && isPrime`?

- (A) `(!isEven || !isPositive) && isPrime`
- (B) `(!isEven || !isPositive) && !isPrime`
- (C) `(!isEven && !isPositive) && isPrime`
- (D) `(!isEven && !isPositive) && !isPrime`

---

## Question 15

Consider the following code segment.

```java
String str = "lookout";
String target = "o";
int j = str.indexOf(target);
while (j >= 0)
{
   str = str.substring(j);
   System.out.print(str + " ");

   str = str.substring(1);
   j = str.indexOf(target);
}
```

What is printed as a result of executing this code segment?

- (A) `ookout okout out`
- (B) `ookout out`
- (C) `okout out`
- (D) `kout ut`

---

## Question 16

The `Book` class will contain attributes for a book's title, author, and number of pages. The class will also contain methods to allow the attributes to be accessed outside the class.

A class design diagram for the `Book` class will contain three sections. The first section contains the class name, the second section contains three instance variables and their data types, and the third section contains three methods and their return types.

The `+` symbol indicates a public designation, and the `-` symbol indicates a private designation.

Of the following diagrams, which represents the most appropriate design for the `Book` class?

**(A)**

```text
+------------------------------+
|             Book             |
+------------------------------+
| + title : String             |
| + author : String            |
| + numPages : String          |
+------------------------------+
| - getTitle() : String        |
| - getAuthor() : String       |
| - getNumPages() : String     |
+------------------------------+
```

**(B)**

```text
+------------------------------+
|             Book             |
+------------------------------+
| - title : String             |
| - author : String            |
| - numPages : String          |
+------------------------------+
| + getTitle() : String        |
| + getAuthor() : String       |
| + getNumPages() : String     |
+------------------------------+
```

**(C)**

```text
+------------------------------+
|             Book             |
+------------------------------+
| + title : String             |
| + author : String            |
| + numPages : int             |
+------------------------------+
| - getTitle() : String        |
| - getAuthor() : String       |
| - getNumPages() : int        |
+------------------------------+
```

**(D)**

```text
+------------------------------+
|             Book             |
+------------------------------+
| - title : String             |
| - author : String            |
| - numPages : int             |
+------------------------------+
| + getTitle() : String        |
| + getAuthor() : String       |
| + getNumPages() : int        |
+------------------------------+
```

---

## Question 17

A programmer developed a mobile application that is intended for users around the world. The programmer wants to maximize the application's system reliability. Which of the following actions is most likely to support this goal?

- (A) Applying security measures to safeguard the personal privacy of users
- (B) Testing the application under a wide variety of possible conditions
- (C) Using `public` visibility for the attributes used in the application
- (D) Using `public` visibility for the behaviors used in the application

---

## Question 18

The following method is intended to return the maximum value that appears in the `int` array `numbers`.

```java
public static int findMaximum(int[] numbers)
{
   int max = 0;
   for (int num : numbers)
   {
      if (num > max)
      {
         max = num;
      }
   }
   return max;
}
```

The method works as intended for some, but not all, `int` arrays. Which of the following best describes the conditions in which the method does **not** return the intended result?

- (A) When the maximum value in `numbers` is the first element in the array
- (B) When the maximum value in `numbers` appears multiple times in the array
- (C) When the maximum value in `numbers` is negative
- (D) When the maximum value in `numbers` is `Integer.MAX_VALUE`

---

## Question 19

Consider the following statement.

```java
int r = (int) (Math.random() * 6) + 10;
```

Which of the following best describes the behavior of the statement?

- (A) It generates a random integer between `6` and `15`, inclusive, and assigns it to `r`.
- (B) It generates a random integer between `6` and `16`, inclusive, and assigns it to `r`.
- (C) It generates a random integer between `10` and `15`, inclusive, and assigns it to `r`.
- (D) It generates a random integer between `10` and `16`, inclusive, and assigns it to `r`.

---

## Question 20

Consider the following code segment.

```java
int value = 100;
while (value % 5 == 0)
{
   value /= 2;
}
System.out.println(value);
```

What is printed as a result of executing this code segment?

- (A) `12`
- (B) `13`
- (C) `25`
- (D) `100`

---

## Question 21

Consider the following code segment.

```java
for (int j = 0; j < 4; j++)
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
0
1 1
2 2 2
3 3 3 3
```

Which of the following can be used to replace `/* missing code */` so that this code segment works as intended?

- (A) `int k = 0; k < j; k++`
- (B) `int k = 0; k <= j; k++`
- (C) `int k = j; k < 4; k++`
- (D) `int k = j; k <= 4; k++`

---

## Question 22

The following class definition does not compile.

```java
public class Pet
{
   private String name;
   private String species;
   private int age;

   public Pet(String pName, String pSpecies)
   {
      name = pName;
      species = pSpecies;
      age = 0;
   }

   public void incrementAge()
   {
      age++;
   }

   public String getInfo()
   {
      return pName + ", " + pSpecies;
   }
}
```

Which of the following best explains why the class will not compile?

- (A) The class has three instance variables, but its constructor has only two parameters.
- (B) The instance variables should be designated `public` instead of `private`.
- (C) The return type of the `incrementAge` method should be `int` instead of `void`.
- (D) The `getInfo` method does not have access to variables named `pName` or `pSpecies`.

---

## Question 23

Consider the following method.

```java
public static void updateArr(int[] arr, int val)
{
   arr[val] = val;
}
```

The following code segment appears in a method in the same class as `updateArr`.

```java
int[] numArr = new int[4];
updateArr(numArr, 1);
updateArr(numArr, 3);
```

What are the contents of `numArr` after executing this code segment?

- (A) `{0, 0, 0, 0}`
- (B) `{0, 1, 0, 3}`
- (C) `{1, 0, 3, 0}`
- (D) `{1, 3, 0, 0}`

---

## Question 24

Consider the following code segment.

```java
ArrayList<Integer> numbers = new ArrayList<Integer>();
numbers.add(100);
numbers.add(200);
numbers.add(0, 300);
numbers.set(2, 400);
numbers.add(2, 500);
```

What are the contents of `numbers` after executing this code segment?

- (A) `[100, 200, 500, 400]`
- (B) `[100, 200, 400, 500]`
- (C) `[300, 100, 400, 500]`
- (D) `[300, 100, 500, 400]`

---

## Question 25

Consider the following class definition.

```java
public class Shoe
{
   private static int count = 0;

   private double size;
   private String style;

   public Shoe()
   {
      size = 7.0;
      style = "boot";
   }

   public Shoe(double mySize, String myStyle)
   {
      size = mySize;
      style = myStyle;
      count++;
   }

   public static int getCount()
   {
      return count;
   }
}
```

The following code segment appears in a class other than `Shoe`.

```java
Shoe s1 = new Shoe(8.5, "sneaker");
Shoe s2 = new Shoe();
Shoe s3 = new Shoe(7.0, "sandal");
System.out.println(Shoe.getCount());
```

What is printed as a result of executing this code segment if it is executed before any other instances of `Shoe` are constructed?

- (A) `0`
- (B) `1`
- (C) `2`
- (D) `3`

---

## Question 26

In the following code segment, `n` is a properly declared and initialized `int` variable whose value is positive.

```java
while (n > 0)      // Line 1
{                  // Line 2
   n /= 10;        // Line 3
}                  // Line 4
System.out.println(n);   // Line 5
```

This code segment is intended to print the leftmost digit of `n`. For example, if `n` is `302`, this code segment should print `3`. However, this code segment is not working as intended. Which of the following changes can be made so that this code segment works as intended?

- (A) In line 1, changing the condition `n > 0` to `n / 10 > 0`
- (B) In line 1, changing the condition `n > 0` to `n % 10 > 0`
- (C) In line 3, changing the statement `n /= 10` to `n -= 10`
- (D) In line 3, changing the statement `n /= 10` to `n %= 10`

---

## Question 27

A school offers optional courses and clubs to students. A programmer wants to investigate whether students at the school who took a computer science course were more or less likely to participate in the school's photography club. The programmer has the following data sets available.

- **Data set 1** contains an entry for each club offered by the school. Each entry includes the name of the club and a list of the names of the club members.
- **Data set 2** contains an entry for each member of the photography club. Each entry contains the name of the member, the number of photographs taken by the member, and the member's favorite course.
- **Data set 3** contains an entry for each student at the school. Each entry contains the name of the student, a list of the courses the student took, and a list of the clubs the student participated in.
- **Data set 4** contains an entry for each student at the school. Each entry contains the name of the student, the student's favorite course, and the student's favorite club.

Which of the data sets is most appropriate for the programmer's investigation?

- (A) Data set 1
- (B) Data set 2
- (C) Data set 3
- (D) Data set 4

---

## Question 28

Consider the following code segment.

```java
String str = "abbcdddeff";
int j = 0;
int count = 0;
while (j < str.length() - 1)
{
   if (str.substring(j, j + 1)
         .equals(str.substring(j + 1, j + 2)))
   {
      count++;
   }
   j++;
}
System.out.println(count);
```

What is printed as a result of executing this code segment?

- (A) `2`
- (B) `3`
- (C) `4`
- (D) `6`

---

## Question 29

Consider the following method.

```java
public static void printNums(int n)
{
   if (n < 50)
   {
      printNums(n * -2);
      System.out.print(n + " ");
   }
}
```

What is printed as a result of the call `printNums(10)`?

- (A) `-80 40 -20 10`
- (B) `10 -20 40 -80`
- (C) `40 -20 10`
- (D) `10 -20 40`

---

## Question 30

The following incomplete method is intended to return the minimum value among its three parameters.

```java
public static int minOfThree(int a, int b, int c)
{
   /* missing code */
}
```

Which of the following can be used to replace `/* missing code */` so that the method works as intended?

**(A)**

```java
if (a <= b)
{
   if (a <= c)
   {
      return a;
   }
   else
   {
      return c;
   }
}
else
{
   return b;
}
```

**(B)**

```java
if (a <= b)
{
   if (a <= c)
   {
      return a;
   }
   else if (b <= c)
   {
      return b;
   }
}
else
{
   return c;
}
```

**(C)**

```java
if (a <= b || a <= c)
{
   return a;
}
else if (b <= a || b <= c)
{
   return b;
}
else
{
   return c;
}
```

**(D)**

```java
if (a <= b && a <= c)
{
   return a;
}
else if (b <= a && b <= c)
{
   return b;
}
else
{
   return c;
}
```

---

## Question 31

A software company wants to develop new features to attract new users to download a popular application. The company created a survey to ask users how they feel about several potential new features. The company sent the survey to all users who spent more than 500 hours using the application during the previous year. A programmer for the company wants to determine which new features should be prioritized by performing an analysis of the survey data. Which of the following best explains why the programmer should be careful about the conclusions drawn from the analysis?

- (A) The survey data may be biased in favor of a specific group of users.
- (B) The survey data may include responses from users who dislike the application.
- (C) The survey data may raise intellectual property concerns for the company.
- (D) The survey data may require data encapsulation before they can be analyzed.

---

## Question 32

Consider the following code segment.

```java
int[][] mat = {{10, 20, 30},
               {40, 50, 60}};

ArrayList<Integer> result = new ArrayList<Integer>();

/* missing code */
```

After executing this code segment, `result` should contain `[30, 20, 10, 60, 50, 40]`.

Which of the following can replace `/* missing code */` to produce this result?

- (A)
  ```java
  for (int j = 0; j < mat.length; j++)
  {
     for (int k = 0; k <= mat[0].length; k++)
     {
        result.add(mat[j][k]);
     }
  }
  ```
- (B)
  ```java
  for (int j = 0; j < mat.length; j++)
  {
     for (int k = mat[0].length - 1; k >= 0; k--)
     {
        result.add(mat[j][k]);
     }
  }
  ```
- (C)
  ```java
  for (int j = mat.length - 1; j >= 0; j--)
  {
     for (int k = 0; k <= mat[0].length; k++)
     {
        result.add(mat[j][k]);
     }
  }
  ```
- (D)
  ```java
  for (int j = mat.length - 1; j >= 0; j--)
  {
     for (int k = mat[0].length - 1; k >= 0; k--)
     {
        result.add(mat[j][k]);
     }
  }
  ```

---

## Question 33

Consider the following code segment.

```java
int[] arr = {10, 20, 30, 40, 50, 60};
for (int j = arr.length - 1; j >= 0; j -= 2)
{
   arr[j]++;
}
```

What are the contents of `arr` after executing this code segment?

- (A) `{10, 20, 31, 40, 51, 60}`
- (B) `{10, 21, 30, 41, 50, 61}`
- (C) `{11, 20, 31, 40, 51, 60}`
- (D) `{11, 21, 31, 41, 51, 61}`

---

## Question 34

Consider the following code segment.

```java
ArrayList<String> oldList = new ArrayList<String>();
oldList.add("a");
oldList.add("b");
oldList.add("c");

ArrayList<String> newList = new ArrayList<String>();

/* missing code */
```

After executing this code segment, `newList` should contain `["c", "b", "a", "a", "b", "c"]`.

Which of the following can replace `/* missing code */` to produce this result?

- (A)
  ```java
  for (String s : oldList)
  {
     newList.add(s);
     newList.add(s);
  }
  ```
- (B)
  ```java
  for (String s : oldList)
  {
     newList.add(s);
     newList.add(0, s);
  }
  ```
- (C)
  ```java
  for (int j = 0; j < oldList.size(); j++)
  {
     newList.add(oldList.get(0));
     newList.add(oldList.get(j));
  }
  ```
- (D)
  ```java
  for (int j = 0; j < oldList.size(); j++)
  {
     newList.add(j, oldList.get(j));
     newList.add(0, oldList.get(j));
  }
  ```

---

## Question 35

Consider the following method.

```java
public static boolean mystery(ArrayList<Integer> numbers)
{
   int prev = Integer.MAX_VALUE;
   for (int num : numbers)
   {
      if (num > prev)
      {
         return false;
      }
      prev = num;
   }
   return true;
}
```

Which of the following best describes the behavior of the method?

- (A) It returns `false` if `numbers` is sorted from least to greatest and returns `true` otherwise.
- (B) It returns `false` if `numbers` is sorted from greatest to least and returns `true` otherwise.
- (C) It returns `true` if `numbers` is sorted from least to greatest and returns `false` otherwise.
- (D) It returns `true` if `numbers` is sorted from greatest to least and returns `false` otherwise.

---

## Question 36

Consider the following code segment.

```java
int[][] my2Darr = new int[4][3];

for (int r = 0; r < my2Darr.length; r++)
{
   for (int c = 0; c < my2Darr[0].length; c++)
   {
      if (r == c)
      {
         my2Darr[r][c] = r;
      }
   }
}
```

Which of the following represents the contents of `my2Darr` after executing this code segment?

- (A)
  ```
  {{0, 0, 0},
   {0, 0, 0},
   {0, 2, 0},
   {0, 0, 3}}
  ```
- (B)
  ```
  {{0, 0, 0},
   {0, 1, 0},
   {0, 0, 2},
   {0, 0, 0}}
  ```
- (C)
  ```
  {{0, 0, 0, 0},
   {0, 0, 2, 0},
   {0, 0, 0, 3}}
  ```
- (D)
  ```
  {{0, 0, 0, 0},
   {0, 1, 0, 0},
   {0, 0, 2, 0}}
  ```

---

## Question 37

Consider the following method. When an element of the two-dimensional array `values` is accessed, the first index is used to specify the row and the second index is used to specify the column.

```java
public static int enigma(int[][] values, int x, int target)
{
   int result = 0;
   for (int num : values[x])
   {
      if (num == target)
      {
         result++;
      }
   }
   return result;
}
```

Which of the following best describes the value returned by the method?

- (A) The index of a column in `values` that contains at least one element equal to `target`
- (B) The index of a row in `values` that contains at least one element equal to `target`
- (C) The number of times that an element equal to `target` appears in column `x` of `values`
- (D) The number of times that an element equal to `target` appears in row `x` of `values`

---

## Question 38

A text file named `sourceText.txt` has the following contents. Assume that the file contains no spaces.

```text
finest_artist
biggest_gamer
nicest_teacher
best_athlete
```

In the following code segment, a valid `Scanner` object named `sc` has been created to read from the text file.

```java
File myText = new File("sourceText.txt");
Scanner sc = new Scanner(myText);

ArrayList<String> firstList = new ArrayList<String>();
ArrayList<String> secondList = new ArrayList<String>();

/* missing code */
```

This code segment is intended to assign `["finest", "biggest", "nicest", "best"]` to `firstList` and to assign `["artist", "gamer", "teacher", "athlete"]` to `secondList`.

Which of the following can replace `/* missing code */` so that this code segment works as intended?

- (A)
  ```java
  while (sc.hasNext())
  {
     firstList.add(sc.next());
     secondList.add(sc.next());
  }
  ```
- (B)
  ```java
  while (sc.hasNext())
  {
     firstList.add(sc.next());
  }
  while (sc.hasNext())
  {
     secondList.add(sc.next());
  }
  ```
- (C)
  ```java
  while (sc.hasNext())
  {
     String[] temp = sc.next().split("_");
     firstList.add(temp[0]);
     secondList.add(temp[1]);
  }
  ```
- (D)
  ```java
  while (sc.hasNext())
  {
     String[] temp = sc.next().split("_");
     firstList.add(temp[0]);
     firstList.add(temp[1]);
  }
  while (sc.hasNext())
  {
     String[] temp = sc.next().split("_");
     secondList.add(temp[0]);
     secondList.add(temp[1]);
  }
  ```

---

## Question 39

The following method implements a recursive binary search algorithm.

```java
public static int binarySearch(int[] elements, int low,
                               int high, int target)
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
int[] numbers = {10, 10, 10, 20, 20, 30, 40, 50, 50, 60, 80};
int result = binarySearch(numbers, 0, numbers.length - 1, 10);
```

What is the value of `result` after executing the code segment?

- (A) `-1`
- (B) `0`
- (C) `1`
- (D) `2`

---

## Question 40

The following method is a correct implementation of the insertion sort algorithm. The method correctly sorts the elements of `numList` so that they appear in order from least to greatest.

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

Assume that `insertionSort` has been called with an `ArrayList` parameter that has been initialized with the following `Integer` objects.

```
[60, 30, 10, 20, 50, 40]
```

What will the contents of `numList` be after three passes of the outer loop (i.e., when `j == 3` at the point indicated by `/* end of outer loop */`)?

- (A) `[10, 20, 30, 50, 60, 40]`
- (B) `[10, 20, 30, 60, 50, 40]`
- (C) `[10, 30, 60, 20, 50, 40]`
- (D) `[20, 10, 30, 60, 50, 40]`

---

## Question 41

Consider the following code segment.

```java
String[] colors = {"red", "green", "orange", "black",
                    "yellow", "brown"};
int result = -1;
for (int j = 0; j < colors.length; j++)
{
   for (int k = j + 1; k < colors.length; k++)
   {
      if (colors[j].length() == colors[k].length())
      {
         result = k;
      }
   }
}
```

What is the value of `result` after executing this code segment?

- (A) `-1`
- (B) `3`
- (C) `4`
- (D) `5`

---

## Question 42

Consider the following code segment.

```java
int j = 0;
int sum = 0;

while (j <= 5)
{
   j++;
   sum += j;
}
```

Which of the following code segments assigns the same value to `sum` as the preceding code segment?

- (A)
  ```java
  int sum = 0;
  for (int j = 0; j <= 5; j++)
  {
     j++;
     sum += j;
  }
  ```
- (B)
  ```java
  int sum = 0;
  for (int j = 0; j <= 6; j++)
  {
     j++;
     sum += j;
  }
  ```
- (C)
  ```java
  int sum = 0;
  for (int j = 1; j <= 5; j++)
  {
     sum += j;
  }
  ```
- (D)
  ```java
  int sum = 0;
  for (int j = 1; j <= 6; j++)
  {
     sum += j;
  }
  ```
