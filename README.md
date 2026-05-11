# Memory Assignment - 15 points

## Background
In this assignment, we will be looking at memory-related tasks for C++ and Rust.

### C++
For the `C++` section, we will fix issues with raw pointers. We will use the code analyzer called `Valgrind` which is a memory-profiling program that can detect memory leaks.

After we fix our issues with raw pointers, we will take a look at the same implementation of `main.cpp` but with **smart pointers**.
You will also use Valgrind to analyze this implementation.

### Rust
For the `Rust` section, we will practice implementing a basic warehouse management system. This main goal is to practice the main ideas of ownership, moving, and borrowing in Rust. You will be asked to modify, evaluate, and write code.

## C++: Pointers

In this section, you will analyze a Gradebook system that uses pointer to a pointer to represent an array of dynamically allocated `doubles`. This pattern is a common source of memory leaks in C++ code.

### Testing 
1. Compile the code and run it through Valgrind:
``` bash
./build.sh main.cpp
```
Observe the "definitely lost" count.

### Part #1: Memory Leaks
The `Exam` class is currently leaking memory in two distinct ways.
#### 1. **Resize Leak** 
In the resize() method, a new array is allocated and the old pointer is reassigned:
``` cpp
newArray[i] = examScores[i];
examScores = newArray; 
```
Because `delete[]` is never called on the old `examScores` array before reassigning it, that memory becomes "orphaned."

**Fix the code and test it as mentioned above.**
#### 2. **Missing Destructor**
Each individual score is allocated with `new double(score)`. Since there is no destructor to `delete` these pointers, every single score added is a permanent leak once the object is no longer referenced.

**Fix the code and test it as mentioned above.**
### Part #2: Smart pointers
For this part, you will not have to write any code. I want you to look at the code provided and take a look at the use of smart pointers. You will see despite the fact that we do not explicitly deallocate/free memory, there are no memory leaks. You can confirm this by compiling with the `./bash` script so that it uses Valgrind to analyze the resulting binary.

#### **Answer these questions:**
1. What type of smart pointer are we using and why?
2. How does this type of smart pointer prevent memory leakage in our program?


## Rust: Ownership and Borrowing

In this section, you will practice implementing a basic Warehouse Management System. Unlike C++, where you must manually manage memory or opt-in to smart pointers, Rust enforces memory safety at compile time using the **Borrow Checker**. Your goal is to satisfy the compiler's strict rules regarding ownership, moving, and borrowing.

### Testing 
1. Compile and run the Rust code through our build script:
``` bash
./build.sh main.rs
```
If your code violates ownership rules, the Rust compiler will immediately stop and provide an error. Your goal is to get the program to compile and run the final integrity check.

### Part #1: Fixing the Borrow Checker 
The `Warehouse` implementation currently has two methods that violate Rust's ownership and borrowing rules.

#### **Task 1: "Move" Trap**
Look at the `list_inventory` method:
``` rust
pub fn list_inventory(self) {
    // ...
}
```
This code is incorrect.

**Explain the reasoning and fix the code. Make sure you test it.**

#### **Task 2: Immutable vs. Mutable Borrows**
Look at the `restock` method:
``` rust
pub fn restock(&self, item_name: &str, amount: i32) {
    for item in self.inventory.iter_mut() {
        // ...
    }
}
```
This code is incorrect.

**Explain the reasoning and fix the code. Make sure you test it.**


#### **Task 3: Ownership Transfer**
Look at the `ship_item` method:
``` rust
pub fn ship_item(&mut self, index: usize) -> Item {
    self.inventory.remove(index)
}
```
**Answer this question:** Explain whether this method is correct or not? In your answer, explain what happens to the ownership of the `Item` when `remove(index)` is called and returned to the caller.

#### **Task 4: Adding an Item**
Create a new method inside the `Warehouse` `impl` block called `add_item`.
* It needs to modify the warehouse's inventory.
* It must accept a parameter of type `Item` - should it be moved or borrowed?.
* It should push the item onto the `inventory` vector.

**Write the method and uncomment the Task 4 test in `main()`.**

#### **Task 5:**
Implement the standalone function `order_popular_items`. 

This function should loop through the `source` warehouse. If an item has a quantity greater than 100, add it to the `dest` warehouse. 

**Implement the function and the Task 5 test in `main()`.**

# Submissions in a PDF

## C++ Part 1 - 3 points
1. Screenshot of `resize` code
2. Screenshot of `destructor` code
3. Screenshot of compilation & execution where `Valgrind` shows memory leaks are fixed
## C++ Part 2 - 2 points
1. Answer to both questions
## Rust - 10 points
### Tasks - 7.5 points(1.5 Points per Task)
1. Task #1: Screenshot of fixed code
2. Task #1: Explanation, per the question/prompt for that section
3. Task #2: Screenshot of fixed code
4. Task #2: Explanation, per the question/prompt for that section
5. Task #3: Explanation to the question/prompt.
6. Task #4: Screenshot of the code
7. Task #5: Screenshot of the code
### Main - 2.5 points for correct output
1. Screenshot of program compiling and running as expected(matching outputs).

