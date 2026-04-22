{
  "version": 1,
  "savedAt": "2026-04-20T19:05:06.182336+00:00",
  "questions": [
    {
      "id": "q1",
      "text": "Consider the following code segment.\n```java\nint x = 20;\nint y = 10;\nint temp = x;\nx = y;\ny = temp * x;\nSystem.out.println(x + \" \" + y);\n```\n What is printed as a result of executing this code segment?",
      "answers": [
        {
          "text": "10 100",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "10 200",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "20 100",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "20 200",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q2",
      "text": "In the following statement, j and k are properly declared and initialized int variables.\n```java\nboolean result = ((j > 0) && (k > 0)) || ((j < 0) && (k < 0));\n```\n  Which of the following best describes the conditions in which result is assigned the value true?",
      "answers": [
        {
          "text": "When j and k are both equal to 0",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "When j and k are both positive or both negative",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "When j is greater than k",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "When j is positive and k is negative or when j is negative and k is positive",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q3",
      "text": "Consider the following Painting class.\n```java\npublic class Painting\n{\n private int year;\n private String artist;\n private String title;\n public Painting(int y, String a)\n {\n year = y;\n artist = a;\n title = \"Untitled\";\n }\n // There are no other constructors.\n}\n```\n  Which of the following code segments, appearing in a class other than Painting, will correctly\ncreate an instance of a Painting object?",
      "answers": [
        {
          "text": "```java\nPainting p = new Painting(\"Frida Kahlo\", \"Self Portrait\", 1939);\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nPainting p = new Painting(1939, \"Frida Kahlo\", \"Self Portrait\");\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nPainting p = new Painting(\"Frida Kahlo\", 1939);\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nPainting p = new Painting(1939, \"Frida Kahlo\");\n```",
          "is_correct": true,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q4",
      "text": "Consider the following code segment.\n```java\nint first = 5 + 10 * 2;\nint second = first + first % 2;\n```\n What is the value of second after this code segment is executed?",
      "answers": [
        {
          "text": "37",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "30",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "26",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "0",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q5",
      "text": "Consider the following code segment.\n```java\ndouble cost = 25.0;\ndouble discount = 0.0;\nif (cost >= 100.0)\n{\n discount = 10.0;\n}\nelse if (cost >= 50.0)\n{\n discount = 5.0;\n}\nelse\n{\n discount = 2.0;\n}\nSystem.out.println(cost - discount);\n```\n What is printed as a result of executing this code segment?",
      "answers": [
        {
          "text": "25.0",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "23.0",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "20.0",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "15.0",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q6",
      "text": "Consider the following code segment.\n```java\nfor (int j = 10; j >= 0; j = 2)   // Line 1-\n{                                  // Line 2\n   System.out.print(j - 1);        // Line 3\n}                                  // Line 4\n```\n  This code segment is intended to produce only the following output. It does not work as intended.\n  Which of the following changes can be made so that this code segment works as intended?",
      "answers": [
        {
          "text": "In line 1, changing the condition j >= 0 to j > 0",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "In line 1, changing the condition j = 10 to j = 9",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "In line 3, changing the condition j - 1 to j + 1",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "In line 3, changing the condition j - 1 to j - 2",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q7",
      "text": "Consider the following code segment.\n```java\nString original = \"DEFGHI\";\nString result = /* missing code */;\nSystem.out.println(result);\n```\n  Which of the following expressions can be used to replace /* missing code */ so that this\ncode segment prints the string \"HIDE\"?",
      "answers": [
        {
          "text": "```java\noriginal.substring(4) + original.substring(0, 1)\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\noriginal.substring(4) + original.substring(0, 2)\n```",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "```java\noriginal.substring(5) + original.substring(1, 2)\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\noriginal.substring(5) + original.substring(1, 3)\n```",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q8",
      "text": "Consider the following code segment.\n```java\ndouble w = 2.0;\ndouble x = 5.0;\ndouble y = (int) w / x;\ndouble z = (int) (w / x);\nSystem.out.println(y + \", \" + z);\n```\n What is printed as a result of executing this code segment?",
      "answers": [
        {
          "text": "0.0, 0.0",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "0.0, 0.4",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "0.4, 0.0",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "0.4, 0.4",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q9",
      "text": "Consider the following class definitions.\n```java\npublic class Measurement\n{\n private int feet;\n private int inches;\n public Measurement(int numFeet, int numInches)\n {\n feet = numFeet;\n inches = numInches;\n }\n private int toInches()\n {\n return feet * 12 + inches;\n }\n}\npublic class Calculations\n{\n public static boolean compare(Measurement m1, Measurement m2)\n {\n if (m1.toInches() >= m2.toInches())\n {\n  return true;\n }\n return false;\n }\n}\n```\n  Which of the following best explains why an error occurs when the classes are compiled?",
      "answers": [
        {
          "text": "The compare method cannot have parameters of type Measurement.",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "The compare method should be declared as void.",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "The toInches method cannot be accessed from a class other than Measurement.",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "The variables feet and inches cannot be accessed from the method toInches.",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q10",
      "text": "A computer game should print different messages based on a player’s score.\n• \"Great!\" is printed for scores greater than 500.\n• \"Good!\" is printed for scores between 250 and 500, inclusive.\n• \"Try again.\" is printed for scores less than 250.\nAssume that score is a properly declared and initialized int that represents a\nplayer’s score.\nWhich of the following code segments will print the correct message for any valid\nvalue of score?",
      "answers": [
        {
          "text": "```java\nString message = \"\";\nif (score < 250)\n{\n   message = \"Try again.\";\n}\nelse if (score <= 500)\n{\n   message = \"Good!\";\n}\nelse\n{\n   message = \"Great!\";\n}\nSystem.out.println(message);\n```",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "```java\nString message = \"\";\nif (score < 250)\n{\n   message = \"Try again.\";\n}\nif (score <= 500)\n{\n   message = \"Good!\";\n}\nelse\n{\n   message = \"Great!\";\n}\nSystem.out.println(message);\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nString message = \"\";\nif (score >= 250)\n{\n   message = \"Good!\";\n}\nelse if (score > 500)\n{\n   message = \"Great!\";\n}\nelse\n{\n   message = \"Try again.\";\n}\nSystem.out.println(message);\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nString message = \"\";\nif (score > 500)\n{\n   message = \"Great!\";\n}\nelse if (score <= 500)\n{\n   message = \"Good!\";\n}\nelse\n{\n   message = \"Try again.\";\n}\nSystem.out.println(message);\n```",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q11",
      "text": "Assume that myPair is a properly declared and initialized Pair object that\nappears in a class other than Pair. Which of the following code segments will\ncompile without error?",
      "answers": [
        {
          "text": "```java\nint val = myPair.getX();\n```",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "```java\nint val = myPair.getX(x);\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nint val = myPair.x;\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nint val = myPair.x1;\n```",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q12",
      "text": "Which of the following methods can be added to the Pair class to allow other\nclasses to modify the value of y?",
      "answers": [
        {
          "text": "```java\nprivate int setY(int num)\n {\n return num;\n }\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\npublic int setY(int num)\n {\n return num;\n }\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nprivate void setY(int num)\n {\n y = num;\n }\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\npublic void setY(int num)\n {\n y = num;\n }\n```",
          "is_correct": true,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q13",
      "text": "Consider the following code segment.\n```java\nfor (int outer = 0; outer < 5; outer++)\n{\n   for (int inner = 4; inner >= 0; inner--)\n   {\n      System.out.println(\"*\");               // Line 5\n   }\n}\n```\n  How many times will the statement in line 5 be executed as a result of running this\ncode segment?",
      "answers": [
        {
          "text": "20",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "24",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "25",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "30",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q14",
      "text": "Assume that isEven, isPositive, and isPrime are boolean variables\nthat have been properly declared and initialized. Which of the following expressions is\nequivalent to the expression\n !(isEven && isPositive) && isPrime?",
      "answers": [
        {
          "text": "(!isEven || !isPositive) && isPrime",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "(!isEven || !isPositive) && !isPrime",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "(!isEven && !isPositive) && isPrime",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "(!isEven && !isPositive) && !isPrime",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q15",
      "text": "Consider the following code segment.\n```java\nString str = \"lookout\";\nString target = \"o\";\nint j = str.indexOf(target);\nwhile (j >= 0)\n{\n   str = str.substring(j);\n   System.out.print(str + \" \");\n   str = str.substring(1);\n   j = str.indexOf(target);\n}\n```\n What is printed as a result of executing this code segment?",
      "answers": [
        {
          "text": "ookout okout out",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "ookout out",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "okout out",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "kout ut",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q16",
      "text": "The Book class will contain attributes for a book’s title, author, and number of pages. The class will also contain methods to allow the attributes to be accessed outside the class.\n\nA class design diagram for the Book class will contain three sections. The first section contains the class name, the second section contains three instance variables and their data types, and the third section contains three methods and their return types.\n\nThe + symbol indicates a public designation, and the - symbol indicates a private designation.\n\nOf the following diagrams, which represents the most appropriate design for the Book class?",
      "answers": [
        {
          "text": "Book with public fields `title : String`, `author : String`, `numPages : String`, and private methods `getTitle() : String`, `getAuthor() : String`, `getNumPages() : String`",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "Book with private fields `title : String`, `author : String`, `numPages : String`, and public methods `getTitle() : String`, `getAuthor() : String`, `getNumPages() : String`",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "Book with public fields `title : String`, `author : String`, `numPages : int`, and private methods `getTitle() : String`, `getAuthor() : String`, `getNumPages() : int`",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "Book with private fields `title : String`, `author : String`, `numPages : int`, and public methods `getTitle() : String`, `getAuthor() : String`, `getNumPages() : int`",
          "is_correct": true,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q17",
      "text": "A programmer developed a mobile application that is intended for users around the\nworld. The programmer wants to maximize the application’s system reliability. Which\nof the following actions is most likely to support this goal?",
      "answers": [
        {
          "text": "Applying security measures to safeguard the personal privacy of users",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "Testing the application under a wide variety of possible conditions",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "Using public visibility for the attributes used in the application",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "Using public visibility for the behaviors used in the application",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q18",
      "text": "The following method is intended to return the maximum value that appears in the\n`int` array `numbers`.\n```java\npublic static int findMaximum(int[] numbers).\npublic static int findMaximum(int[] numbers)\n{\n   int max = 0;\n   for (int num : numbers)\n   {\n      if (num > max)\n      {\n         max = num;\n      }\n   }\n   return max;\n}\n```\n  The method works as intended for some, but not all, int arrays. Which of the\nfollowing best describes the conditions in which the method does not return the\nintended result?",
      "answers": [
        {
          "text": "When the maximum value in numbers is the first element in the array",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "When the maximum value in numbers appears multiple times in the array",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "When the maximum value in numbers is negative",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "When the maximum value in numbers is Integer.MAX_VALUE",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q19",
      "text": "Consider the following statement.\n```java\nint r = (int) (Math.random() * 6) + 10;\n```\n Which of the following best describes the behavior of the statement?",
      "answers": [
        {
          "text": "It generates a random integer between 6 and 15, inclusive, and assigns it to r.",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "It generates a random integer between 6 and 16, inclusive, and assigns it to r.",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "It generates a random integer between 10 and 15, inclusive, and assigns it to r.",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "It generates a random integer between 10 and 16, inclusive, and assigns it to r.",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q20",
      "text": "Consider the following code segment.\n```java\nint value = 100;\nwhile (value % 5 == 0)\n{\n   value /= 2;\n}\nSystem.out.println(value);\n```\n What is printed as a result of executing this code segment?",
      "answers": [
        {
          "text": "12",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "13",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "25",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "100",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q21",
      "text": "Consider the following code segment.\n```java\nfor (int j = 0; j < 4; j++)\n{\n   for (/* missing code */)\n   {\n      System.out.print(j + \" \");\n   }\n   System.out.println();\n}\n```\n This code segment is intended to produce the following output.\n1 1\n2 2 2\n3 3 3 3\n  Which of the following can be used to replace /* missing code */ so that\nthis code segment works as intended?",
      "answers": [
        {
          "text": "```java\nint k = 0; k < j; k++\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nint k = 0; k <= j; k++\n```",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "```java\nint k = j; k < 4; k++\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nint k = j; k <= 4; k++\n```",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q22",
      "text": "The following class definition does not compile.\n```java\npublic class Pet\n{\n   private String name;\n   private String species;\n   private int age;\n   public Pet(String pName, String pSpecies)\n   {\n      name = pName;\n      species = pSpecies;\n      age = 0;\n   }\n   public void incrementAge()\n   {\n      age++;\n   }\n   public String getInfo()\n   {\n      return pName + \", \" + pSpecies;\n   }\n}\n```\n Which of the following best explains why the class will not compile?",
      "answers": [
        {
          "text": "The class has three instance variables, but its constructor has only two\nparameters.",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "The instance variables should be designated public instead of private.",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "The return type of the incrementAge method should be int instead of `void`.",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "The getInfo method does not have access to variables named pName or\npSpecies.",
          "is_correct": true,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q23",
      "text": "Consider the following method.\n```java\npublic static void updateArr(int[] arr, int val)\n{\n   arr[val] = val;\n}\n```\n The following code segment appears in a method in the same class as updateArr.\n```java\nint[] numArr = new int[4];\nupdateArr(numArr, 1);\nupdateArr(numArr, 3);\n```\n What are the contents of numArr after executing this code segment?",
      "answers": [
        {
          "text": "{0, 0, 0, 0}",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "{0, 1, 0, 3}",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "{1, 0, 3, 0}",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "{1, 3, 0, 0}",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q24",
      "text": "Consider the following code segment.\n```java\nArrayList<Integer> numbers = new ArrayList<Integer>();\nnumbers.add(100);\nnumbers.add(200);\nnumbers.add(0, 300);\nnumbers.set(2, 400);\nnumbers.add(2, 500);\n```\n What are the contents of numbers after executing numbers this code segment?",
      "answers": [
        {
          "text": "[100, 200, 500, 400]",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "[100, 200, 400, 500]",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "[300, 100, 400, 500]",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "[300, 100, 500, 400]",
          "is_correct": true,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q25",
      "text": "Consider the following class definition.\n```java\npublic class Shoe\n{\n   private static int count = 0;\n   private double size;\n   private String style;\n   public Shoe()\n   {\n      size = 7.0;\n      style = \"boot\";\n   }\n   public Shoe(double mySize, String myStyle)\n   {\n      size = mySize;\n      style = myStyle;\n      count++;\n   }\n   public static int getCount()\n   {\n      return count;\n   }\n}\n```\n The following code segment appears in a class other than Shoe.\n```java\nShoe s1 = new Shoe(8.5, \"sneaker\");\nShoe s2 = new Shoe();\nShoe s3 = new Shoe(7.0, \"sandal\");\nSystem.out.println(Shoe.getCount());\n```\n  What is printed as a result of executing this code segment if it is executed before any\nother instances of Shoe are constructed?",
      "answers": [
        {
          "text": "0",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "1",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "2",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "3",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q26",
      "text": "In the following code segment, `n` is a properly declared and initialized `int` variable whose value is positive.\n\n```java\nwhile (n > 0)           // Line 1\n{                       // Line 2\n   n /= 10;             // Line 3\n}                       // Line 4\nSystem.out.println(n);  // Line 5\n```\n\nThis code segment is intended to print the leftmost digit of `n`. For example, if `n` is `302`, this code segment should print `3`. However, this code segment is not working as intended. Which of the following changes can be made so that this code segment works as intended?",
      "answers": [
        {
          "text": "In line 1, changing the condition `n > 0` to `n / 10 > 0`",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "In line 1, changing the condition `n > 0` to `n % 10 > 0`",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "In line 3, changing the statement `n /= 10` to `n -= 10`",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "In line 3, changing the statement `n /= 10` to `n %= 10`",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q27",
      "text": "A school offers optional courses and clubs to students. A programmer wants to\ninvestigate whether students at the school who took a computer science course were\nmore or less likely to participate in the school’s photography club. The programmer has\nthe following data sets available.\n• Data set 1 contains an entry for each club offered by the school. Each entry includes\nthe name of the club and a list of the names of the club members.\n• Data set 2 contains an entry for each member of the photography club. Each entry\ncontains the name of the member, the number of photographs taken by the member,\nand the member’s favorite course.\n• Data set 3 contains an entry for each student at the school. Each entry contains the\nname of the student, a list of the courses the student took, and a list of the clubs the\nstudent participated in.\n• Data set 4 contains an entry for each student at the school. Each entry contains the\nname of the student, the student’s favorite course, and the student’s favorite club.\n Which of the data sets is most appropriate for the programmer’s investigation?",
      "answers": [
        {
          "text": "Data set 1",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "Data set 2",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "Data set 3",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "Data set 4",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q28",
      "text": "Consider the following code segment.\n```java\nString str = \"abbcdddeff\";\nint j = 0;\nint count = 0;\nwhile (j < str.length() - 1)\n{\n   if (str.substring(j, j + 1)\n          .equals(str.substring(j + 1, j + 2)))\n   {\n      count++;\n   }\n   j++;\n}\nSystem.out.println(count);\n```\n What is printed as a result of executing this code segment?",
      "answers": [
        {
          "text": "2",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "3",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "4",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "6",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q29",
      "text": "Consider the following method.\n```java\npublic static void printNums(int n)\n{\n   if (n < 50)\n   {\n      printNums(n * -2);\n      System.out.print(n + \" \");\n   }\n}\n```\n What is printed as a result of the call printNums(10)?",
      "answers": [
        {
          "text": "-80 40 -20 10",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "10 -20 40 -80",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "40 -20 10",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "10 -20 40",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q30",
      "text": "The following incomplete method is intended to return the minimum value among its\nthree parameters.\n```java\npublic static int minOfThree(int a, int b, int c)\n{\n   /* missing code */\n}\n```\n  Which of the following can be used to replace /* missing code */ so that\nthe method works as intended?",
      "answers": [
        {
          "text": "```java\nif (a <= b)\n   {\n      if (a <= c)\n      {\n         return a;\n      }\n      else\n      {\n         return c;\n      }\n   }\n   else\n   {\n      return b;\n   }\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nif (a <= b)\n {\n      if (a <= c)\n      {\n         return a;\n      }\n      else if (b <= c)\n      {\n         return b;\n      }\n   }\n   else\n   {\n      return c;\n   }\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nif (a <= b || a <= c)\n   {\n      return a;\n   }\n   else if (b <= a || b <= c)\n   {\n      return b;\n   }\n   else\n   {\n      return c;\n   }\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nif (a <= b && a <= c)\n   {\n      return a;\n   }\n   else if (b <= a && b <= c)\n   {\n      return b;\n   }\n   else\n   {\n      return c;\n   }\n```",
          "is_correct": true,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q31",
      "text": "A software company wants to develop new features to attract new users to download\na popular application. The company created a survey to ask users how they feel about\nseveral potential new features. The company sent the survey to all users who spent more\nthan 500 hours using the application during the previous year. A programmer for the\ncompany wants to determine which new features should be prioritized by performing an\nanalysis of the survey data. Which of the following best explains why the programmer\nshould be careful about the conclusions drawn from the analysis?",
      "answers": [
        {
          "text": "The survey data may be biased in favor of a specific group of users.",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "The survey data may include responses from users who dislike the application.",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "The survey data may raise intellectual property concerns for the company.",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "The survey data may require data encapsulation before they can be analyzed.",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q32",
      "text": "Consider the following code segment.\n```java\nint[][] mat = {{10, 20, 30},\n               {40, 50, 60}};\nArrayList<Integer> result = new ArrayList<Integer>();\n/* missing code */\n```\n  After executing this code segment, result should contain [30, 20, 10,\n60, 50, 40].\n  Which of the following can replace /* missing code */ to produce this\nresult?",
      "answers": [
        {
          "text": "```java\nfor (int j = 0; j < mat.length; j++)\n   {\n      for (int k = 0; k <= mat[0].length; k++)\n      {\n         result.add(mat[j][k]);\n      }\n   }\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nfor (int j = 0; j < mat.length; j++)\n   {\n      for (int k = mat[0].length - 1; k >= 0; k--)\n      {\n         result.add(mat[j][k]);\n      }\n   }\n```",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "```java\nfor (int j = mat.length - 1; j >= 0; j--)\n   {\n      for (int k = 0; k <= mat[0].length; k++)\n      {\n         result.add(mat[j][k]);\n      }\n   }\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nfor (int j = mat.length - 1; j >= 0; j--)\n   {\n      for (int k = mat[0].length - 1; k >= 0; k--)\n      {\n         result.add(mat[j][k]);\n      }\n   }\n```",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q33",
      "text": "Consider the following code segment.\n```java\nint[] arr = {10, 20, 30, 40, 50, 60};\nfor (int j = arr.length - 1; j >= 0; j -= 2)\n{\n   arr[j]++;\n}\n```\n What are the contents of arr after executing this code segment?",
      "answers": [
        {
          "text": "{10, 20, 31, 40, 51, 60}",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "{10, 21, 30, 41, 50, 61}",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "{11, 20, 31, 40, 51, 60}",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "{11, 21, 31, 41, 51, 61}",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q34",
      "text": "Consider the following code segment.\n```java\nArrayList<String> oldList = new ArrayList<String>();\noldList.add(\"a\");\noldList.add(\"b\");\noldList.add(\"c\");\nArrayList<String> newList = new ArrayList<String>();\n/* missing code */\n```\n  After executing this code segment, newList should contain\n[\"c\", \"b\", \"a\", \"a\", \"b\", \"c\"].\n  Which of the following can replace /* missing code */ to produce this\nresult?",
      "answers": [
        {
          "text": "```java\nfor (String s : oldList)\n   {\n      newList.add(s);\n      newList.add(s);\n   }\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nfor (String s : oldList)\n   {\n      newList.add(s);\n      newList.add(0, s);\n   }\n```",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "```java\nfor (int j = 0; j < oldList.size(); j++)\n   {\n      newList.add(oldList.get(0));\n      newList.add(oldList.get(j));\n   }\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nfor (int j = 0; j < oldList.size(); j++)\n   {\n      newList.add(j, oldList.get(j));\n      newList.add(0, oldList.get(j));\n   }\n```",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q35",
      "text": "Consider the following method.\n```java\npublic static boolean mystery(ArrayList<Integer> numbers)\n{\n   int prev = Integer.MAX_VALUE;\n   for (int num : numbers)\n   {\n      if (num > prev)\n      {\n         return false;\n      }\n      prev = num;\n   }\n   return true;\n}\n```\n Which of the following best describes the behavior of the method?",
      "answers": [
        {
          "text": "It returns false if numbers is sorted from least to greatest and returns\ntrue otherwise.",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "It returns  false if numbers is sorted from greatest to least and returns\ntrue otherwise.",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "It returns true if numbers is sorted from least to greatest and returns\nfalse otherwise.",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "It returns true if numbers is sorted from greatest to least and returns\nfalse otherwise.",
          "is_correct": true,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q36",
      "text": "Consider the following code segment.\n```java\nint[][] my2Darr = new int[4][3];\nfor (int r = 0; r < my2Darr.length; r++)\n{\n   for (int c = 0; c < my2Darr[0].length; c++)\n   {\n      if (r == c)\n      {\n         my2Darr[r][c] = r;\n      }\n   }\n}\n```\n  Which of the following represents the contents of my2Darr after executing this code segment?",
      "answers": [
        {
          "text": "{{0, 0, 0},\n {0, 0, 0},\n {0, 2, 0},\n {0, 0, 3}}",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "{{0, 0, 0},\n {0, 1, 0},\n {0, 0, 2},\n {0, 0, 0}}",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "{{0, 0, 0, 0},\n {0, 0, 2, 0},\n {0, 0, 0, 3}}",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "{{0, 0, 0, 0},\n {0, 1, 0, 0},\n {0, 0, 2, 0}}",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q37",
      "text": "Consider the following method. When an element of the two-dimensional array values is\naccessed, the first index is used to specify the row and the second index is used to specify the\ncolumn.\n```java\npublic static int enigma(int[][] values, int x, int target)\n{\n   int result = 0;\n   for (int num : values[x])\n   {\n      if (num == target)\n      {\n         result++;\n      }\n   }\n   return result;\n}\n```\n Which of the following best describes the value returned by the method?",
      "answers": [
        {
          "text": "The index of a column in values that contains at least one element equal to target",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "The index of a row in values that contains at least one element equal to target",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "The number of times that an element equal to target appears in column x of values",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "The number of times that an element equal to target appears in row x of values",
          "is_correct": true,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q38",
      "text": "A text file named `sourceText.txt` has the following contents. Assume that the file contains no spaces.\n```\nfinest_artist\nbiggest_gamer\nnicest_teacher\nbest_athlete\n```\nIn the following code segment, a valid `Scanner` object named `sc` has been created to read from the text file.\n```java\nFile myText = new File(\"sourceText.txt\");\nScanner sc = new Scanner(myText);\nArrayList<String> firstList = new ArrayList<String>();\nArrayList<String> secondList = new ArrayList<String>();\n/* missing code */\n```\n  This code segment is intended to assign `[\"finest\", \"biggest\", \"nicest\", \"best\"]`\nto `firstList` and to assign `[\"artist\", \"gamer\", \"teacher\", \"athlete\"]`\nto `secondList`.\n  Which of the following can replace `/* missing code */` so that this code segment works as intended?",
      "answers": [
        {
          "text": "```java\nwhile (sc.hasNext())\n   {\n      firstList.add(sc.next());\n      secondList.add(sc.next());\n   }\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nwhile (sc.hasNext())\n   {\n      firstList.add(sc.next());\n   }\n   while (sc.hasNext())\n   {\n      secondList.add(sc.next());\n   }\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nwhile (sc.hasNext())\n   {\n      String[] temp = sc.next().split(\"_\");\n      firstList.add(temp[0]);\n      secondList.add(temp[1]);\n   }\n```",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "```java\nwhile (sc.hasNext())\n   {\n      String[] temp = sc.next().split(\"_\");\n      firstList.add(temp[0]);\n      firstList.add(temp[1]);\n   }\n   while (sc.hasNext())\n   {\n      String[] temp = sc.next().split(\"_\");\n      secondList.add(temp[0]);\n      secondList.add(temp[1]);\n   }\n```",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q39",
      "text": "The following method implements a recursive binary search algorithm.\n```java\npublic static int binarySearch(int[] elements, int low,\n                               int high, int target)\n{\n   if (low <= high)\n   {\n\nint mid = (high + low) / 2;\nif (target < elements[mid])\n\n  {\n          return binarySearch(elements, low, mid - 1, target);\n      }\n      else if (target > elements[mid])\n      {\n          return binarySearch(elements, mid + 1, high, target);\n      }\n      else if (elements[mid] == target)\n      {\n         return mid;\n      }\n   }\n   return -1;\n}\n```\n  The following code segment appears in a method in the same class as binarySearch.\n```java\nint[] numbers = {10, 10, 10, 20, 20, 30, 40, 50, 50, 60, 80};\nint result = binarySearch(numbers, 0, numbers.length - 1, 10);\n```\n What is the value of result after executing the code segment?",
      "answers": [
        {
          "text": "-1",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "0",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "1",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "2",
          "is_correct": true,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q40",
      "text": "The following method is a correct implementation of the insertion sort algorithm. The method\ncorrectly sorts the elements of numList so that they appear in order from least to greatest.\n```java\npublic static void insertionSort(ArrayList<Integer> numList)\n{\n   for (int j = 1; j < numList.size(); j++)\n   {\n      int temp = numList.get(j);\n      int k = j;\n      while (k > 0 && temp < numList.get(k - 1))\n      {\n         numList.set(k, numList.get(k - 1));\nk--;\n\n  }\n      numList.set(k, temp);\n      /* end of outer loop */\n   }\n}\n```\n  Assume that insertionSort has been called with an ArrayList parameter that has\nbeen initialized with the following Integer objects.\n[60, 30, 10, 20, 50, 40]\n  What will the contents of numList be after three passes of the outer loop (i.e., when j == 3\nat the point indicated by /* end of outer loop */)?",
      "answers": [
        {
          "text": "[10, 20, 30, 50, 60, 40]",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "[10, 20, 30, 60, 50, 40]",
          "is_correct": true,
          "explanation": null
        },
        {
          "text": "[10, 30, 60, 20, 50, 40]",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "[20, 10, 30, 60, 50, 40]",
          "is_correct": false,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q41",
      "text": "Consider the following code segment.\n```java\nString[] colors = {\"red\", \"green\", \"orange\", \"black\",\n                   \"yellow\", \"brown\"};\nint result = -1;\nfor (int j = 0; j < colors.length; j++)\n{\n   for (int k = j + 1; k < colors.length; k++)\n   {\n      if (colors[j].length() == colors[k].length())\n      {\n         result = k;\n      }\n   }\n}\n```\n What is the value of result after executing this code segment?",
      "answers": [
        {
          "text": "-1",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "3",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "4",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "5",
          "is_correct": true,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    },
    {
      "id": "q42",
      "text": "Consider the following code segment.\n```java\nint j = 0;\nint sum = 0;\nwhile (j <= 5)\n{\n   j++;\n   sum += j;\n}\n```\n  Which of the following code segments assigns the same value to sum as the\npreceding code segment?",
      "answers": [
        {
          "text": "```java\nint sum = 0;\n   for (int j = 0; j <= 5; j++)\n   {\n      j++;\n      sum += j;\n   }\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nint sum = 0;\n   for (int j = 0; j <= 6; j++)\n   {\n      j++;\n      sum += j;\n   }\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nint sum = 0;\n   for (int j = 1; j <= 5; j++)\n   {\n      sum += j;\n   }\n```",
          "is_correct": false,
          "explanation": null
        },
        {
          "text": "```java\nint sum = 0;\n   for (int j = 1; j <= 6; j++)\n   {\n      sum += j;\n   }\n```",
          "is_correct": true,
          "explanation": null
        }
      ],
      "explanation": null,
      "rubric": null,
      "distractors": null,
      "subject": "Computer Science",
      "topics": [],
      "difficulty": null
    }
  ]
}
