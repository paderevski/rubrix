# LaTeX Support in Rubrix

Rubrix now supports LaTeX math formulas in questions and answers! Use `$...$` for inline math and `$$...$$` for display math.

## How It Works

When you export questions to QTI format, any LaTeX formulas will be automatically converted to image tags that call the learn.lcps.org LaTeX-to-SVG API.

## Examples

### Inline Math

**Input:**
```
What is the value of $3x-sin\left(x\right)+\sqrt{x}$ when x=5?
```

**Output (in QTI XML):**
```html
What is the value of <img src="https://learn.lcps.org/svc/latex/latex-to-svg?latex=%5Clarge%203x-sin%5Cleft%28x%5Cright%29%2B%5Csqrt%7Bx%7D" alt="3x-sin\left(x\right)+\sqrt{x}" formula="3x-sin\left(x\right)+\sqrt{x}" class="mathquill-formula" /> when x=5?
```

### Display Math

**Input:**
```
An equation: $$\large 3x-sin\left(x\right)+\sqrt{x}$$
```

**Output (in QTI XML):**
```html
An equation: <img src="https://learn.lcps.org/svc/latex/latex-to-svg?latex=%5Clarge%203x-sin%5Cleft%28x%5Cright%29%2B%5Csqrt%7Bx%7D" alt="\large 3x-sin\left(x\right)+\sqrt{x}" formula="\large 3x-sin\left(x\right)+\sqrt{x}" class="mathquill-formula" />
```

### In Answers

LaTeX also works in answer choices:

```
a. $x^2 + 2x + 1$
b. $x^2 - 2x + 1$
c. $(x+1)^2$
d. $(x-1)^2$
```

## Sample Question

Here's a complete example of a question with LaTeX:

```
1. What is the derivative of $f(x) = x^3 + 2x^2 - 5x + 3$?

a. $f'(x) = 3x^2 + 4x - 5$
b. $f'(x) = 3x^2 + 2x - 5$
c. $f'(x) = x^2 + 4x - 5$
d. $f'(x) = 3x + 4$
```

When exported to QTI, all the LaTeX formulas will be rendered as images through the learn.lcps.org API, making them compatible with LMS platforms like Schoology.

## Implementation Details

- LaTeX formulas are URL-encoded before being sent to the API
- Display math (`$$...$$`) is processed before inline math (`$...$`)
- The conversion happens in the QTI export step, so your raw questions remain readable
- The generated img tags include:
  - `src`: The API endpoint with URL-encoded formula
  - `alt`: The original LaTeX formula (for accessibility)
  - `formula`: The original LaTeX formula (for MathQuill)
  - `class="mathquill-formula"`: For styling and recognition by LMS platforms
