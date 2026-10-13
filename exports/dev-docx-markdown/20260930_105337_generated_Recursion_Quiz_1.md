# Recursion Quiz 1 - Version 1

::: {custom-style="QuestionBody"}
**Question 1.** Consider the following method. What value is returned by the call `g(5)`?
:::

```java
public int g(int n) {
    if (n <= 0) {
        return 0;
    } else {
        return n * n + g(n - 2);
    }
}
```

A) `15`
B) `35`
C) `55`
D) `9`
E) `36`

::: {custom-style="QuestionBody"}
**Question 2.** Consider the following method. What value is returned by the call `mystery(2, 3)`?
:::

```java
public int mystery(int a, int b) {
    if (b == 0) return a;
    else return mystery(a + b, b - 1);
}
```

A) `6`
B) `7`
C) `5`
D) `9`
E) `8`

::: {custom-style="QuestionBody"}
**Question 3.** Consider the following method. What is the output of the call `mystery2(307)`?
:::

```java
public void mystery2(int y) {
    if (y == 0) {
        System.out.print("*");
        return;
    }
    System.out.print(y % 10);
    mystery2(y / 10);
}
```

A) `703*`
B) `*307`
C) `300 0 7 *`
D) `307`
E) `703`

::: {custom-style="QuestionBody"}
**Question 4.** Consider the following method. What value is returned by the call `foo(4)`?
:::

```java
public static int foo(int n) {
    if (n <= 1) {
        return 1;           // base case
    } else {
        return foo(n - 1) + 2 * foo(n - 2);
    }
}
```

A) `27`
B) `0`
C) `8`
D) `11`
E) `5`

::: {custom-style="QuestionBody"}
**Question 5.** Consider the following method. What value is returned by the call `f(nums, 0)`?
:::

```java
public static int f(int[] data, int index) {
    if (index == data.length) {
        return 0;
    }
    return data[index] + f(data, index + 1);
}

int[] nums = {2, 5, -1, 4};
```

Select the correct result.

A) `6`
B) `12`
C) `10`
D) `13`
E) `8`

::: {custom-style="QuestionBody"}
**Question 6.** Examine the method below. What is printed by the call `pattern(3)`?
:::

```java
public static void pattern(int n) {
    if (n == 0) {
        System.out.print("*");
        return;
    }
    System.out.print(n);      
    pattern(n - 1);           
    System.out.print(n);     
}
```

A) `321*`
B) `*112233`
C) `321*123`
D) `321*123`
E) `123**321`

::: {custom-style="QuestionBody"}
**Question 7.** Consider the following Java program. Which of the following method calls returns `true` when executed? Select **all** that apply.
:::

```java

    public static boolean q(String s) {
        if (s.length() <= 1) {
            return true;
        }
        if (s.charAt(0) != s.charAt(s.length() - 1)) {
            return false;
        }
        return q(s.substring(1, s.length() - 1));
    }
}
```

A) `q(“aa")`
B) `q(“aaabbb")`
C) `q(“12321")`
D) `q("civic")`
E) `q("")`

::: {custom-style="QuestionBody"}
**Question 8.** Consider the following recursive binary‑search method. One essential line has been removed (indicated by the comment). Which line must be inserted so that the method correctly returns the index of **key** in **arr**, or **-1** if the key is absent?
:::

```java
public static int b(int[] arr, int low, int high, int key) {
    if (low > high) {
        return -1;
    }
    int mid = low + (high - low) / 2;
    if (arr[mid] == key) {
        return mid;
    }
    // _______ (missing line)
    else return b(arr, mid + 1, high, key);
}
```

A) X
B) `mid = low + (high - low) / 2;`
C) `int mid = (low + high) / 2;`
D) X
E) X

::: {custom-style="QuestionBody"}
**Question 9.** Consider the following method. What value is returned by the call `foo(5)`?
:::

```java
public int foo(int n) {
    if (n == 1) return 1;
    if (n % 2 == 0)               // even n
        return foo(n-1) + n;
    else                          // odd n (n > 1)
        return foo(n-2) + n * n;
}
```

A) `26`
B) `39`
C) `55`
D) `35`
E) `19`

```{=openxml}
<w:p><w:r><w:br w:type="page"/></w:r></w:p>
```

## Quiz Key -- Version 1

1. B
2. E
3. A
4. D
5. C
6. D
7. D
8. C
9. D


```{=openxml}
<w:p><w:r><w:br w:type="page"/></w:r></w:p>
```

# Recursion Quiz 1 - Version 2

::: {custom-style="QuestionBody"}
**Question 1.** Consider the following recursive binary‑search method. One essential line has been removed (indicated by the comment). Which line must be inserted so that the method correctly returns the index of **key** in **arr**, or **-1** if the key is absent?
:::

```java
public static int b(int[] arr, int low, int high, int key) {
    if (low > high) {
        return -1;
    }
    int mid = low + (high - low) / 2;
    if (arr[mid] == key) {
        return mid;
    }
    // _______ (missing line)
    else return b(arr, mid + 1, high, key);
}
```

A) X
B) `mid = low + (high - low) / 2;`
C) X
D) `int mid = (low + high) / 2;`
E) X

::: {custom-style="QuestionBody"}
**Question 2.** Consider the following method. What is the output of the call `mystery2(307)`?
:::

```java
public void mystery2(int y) {
    if (y == 0) {
        System.out.print("*");
        return;
    }
    System.out.print(y % 10);
    mystery2(y / 10);
}
```

A) `300 0 7 *`
B) `*307`
C) `703`
D) `703*`
E) `307`

::: {custom-style="QuestionBody"}
**Question 3.** Consider the following method. What value is returned by the call `g(5)`?
:::

```java
public int g(int n) {
    if (n <= 0) {
        return 0;
    } else {
        return n * n + g(n - 2);
    }
}
```

A) `36`
B) `15`
C) `35`
D) `9`
E) `55`

::: {custom-style="QuestionBody"}
**Question 4.** Consider the following method. What value is returned by the call `foo(4)`?
:::

```java
public static int foo(int n) {
    if (n <= 1) {
        return 1;           // base case
    } else {
        return foo(n - 1) + 2 * foo(n - 2);
    }
}
```

A) `5`
B) `0`
C) `27`
D) `11`
E) `8`

::: {custom-style="QuestionBody"}
**Question 5.** Examine the method below. What is printed by the call `pattern(3)`?
:::

```java
public static void pattern(int n) {
    if (n == 0) {
        System.out.print("*");
        return;
    }
    System.out.print(n);      
    pattern(n - 1);           
    System.out.print(n);     
}
```

A) `123**321`
B) `321*123`
C) `321*`
D) `321*123`
E) `*112233`

::: {custom-style="QuestionBody"}
**Question 6.** Consider the following method. What value is returned by the call `f(nums, 0)`?
:::

```java
public static int f(int[] data, int index) {
    if (index == data.length) {
        return 0;
    }
    return data[index] + f(data, index + 1);
}

int[] nums = {2, 5, -1, 4};
```

Select the correct result.

A) `8`
B) `6`
C) `12`
D) `13`
E) `10`

::: {custom-style="QuestionBody"}
**Question 7.** Consider the following Java program. Which of the following method calls returns `true` when executed? Select **all** that apply.
:::

```java

    public static boolean q(String s) {
        if (s.length() <= 1) {
            return true;
        }
        if (s.charAt(0) != s.charAt(s.length() - 1)) {
            return false;
        }
        return q(s.substring(1, s.length() - 1));
    }
}
```

A) `q("")`
B) `q("civic")`
C) `q(“aaabbb")`
D) `q(“aa")`
E) `q(“12321")`

::: {custom-style="QuestionBody"}
**Question 8.** Consider the following method. What value is returned by the call `foo(5)`?
:::

```java
public int foo(int n) {
    if (n == 1) return 1;
    if (n % 2 == 0)               // even n
        return foo(n-1) + n;
    else                          // odd n (n > 1)
        return foo(n-2) + n * n;
}
```

A) `35`
B) `19`
C) `39`
D) `26`
E) `55`

::: {custom-style="QuestionBody"}
**Question 9.** Consider the following method. What value is returned by the call `mystery(2, 3)`?
:::

```java
public int mystery(int a, int b) {
    if (b == 0) return a;
    else return mystery(a + b, b - 1);
}
```

A) `9`
B) `7`
C) `5`
D) `6`
E) `8`

```{=openxml}
<w:p><w:r><w:br w:type="page"/></w:r></w:p>
```

## Quiz Key -- Version 2

1. D
2. D
3. C
4. D
5. D
6. E
7. B
8. A
9. E
