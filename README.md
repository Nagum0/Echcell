# USER DOCUMENTATION

### TABLE SETUP
- A table has 2 core parts; the **header** and the **body**;
- The header is necessary because it specifies the column count and is used for **cell pointers** to identify the column index (x coordinate). The header can any 1 letter character. The idiomaitic way is to go in the order of the abc;
- The body holds the data of the table;
- Values must be separated by **commas**;
- If the value's first character is a **=** it will be interpreted as a **expression**;
- A table:
``` csv
A,B,C,D
John Doe,35,Male,IT
Felix Argyle,21,Male,Healthcare
Zack Bene,19,Male,IT
Misato Katsuragi,30,Female,NULL
```

### EXPRESSION RULES
- Expressions are declared by a **=**;
- Every expression must start a **function**;
    - Current functions:
        - SUM: *Returns the sum of a given range*
        - AVG: *Returns the average of a given range*
        - EXPR: *Evaluates a mathematical expression*
- Function **arguments** must be passed after the function call, separated by whitespace;
``` csv
= SUM A1 C1
```

### CELL POINTERS
- A **cell pointer** holds the x and y coordinates for a specific cell (it points to the cell's value);
- The first character of a cell pointer is the column number (x coordinate) and the rest of the characters must be numbers representing the row number (y coordinate);
- **A2**: Here A stands for the first column (x = 1) and the number following A stands for the second row (y = 2);

### RANGES
- Some functions take **ranges** as arguments;
- **Ranges** have a start and an end. Both the start and the end are **cell pointers**;
- *A range is a vector containing all of the cell values from the start to the end of the range including both*;
- Either the column or row index must match on both cell pointers (ranges are either column based or row based; nothing diagonal).
``` csv
// Row based range:
A1 D1
```
``` csv
// Column based range:
A1 A5
```
``` csv
// Incorrect range:
A1 B2
```

# TODO!
- SUM Function - DONE
- Custom error types - DONE
- AVG Function - DONE
- Mathematical expressions - DONE
- IF function - DONE
- SUMIF function - Working on...

### Defined functions:
- **SUM**: Returns the sum of a given range;
```
= SUM <range>
```
- **AVG**: Returns the average of a given range;
```
= AVG <range>
```
- **CALC**: Calculates a mathematical expression;
```
= CALC <expr>
```
- **IF**: Is broken into 3 parts:
    - *Condition*: Checks whether a condition is true or false;
    - *If the condition is true*: Returns the element or expression after THEN;
    - *If the condition is false*: Returns the element or expression after ELSE;
    - Currently condition checking only works on Number types;
```
= IF 1 == 1 THEN SUM A1 A5 ELSE SUM B1 B5
```
