# Linear Algebra Project

## Introductions

This repository contains my implementation of the Linear Algebra module. The goal is to build a robust mathematical foundation by implementing vectors, matrices, and their operations from scratch, without relying on external libraries.

## General Rules

To ensure a rigorous and deep understanding of the underlying mathematical structures, the following constraints are strictly applied:

### Language and Technical Requirements

- **Generic Support**: The implementation must handle different numerical types using generics.
- **First-Class Functions**: The language must support lambda expressions or functions as first-class citizens.
- **Operator Overloading (Optional)**: Highly encouraged to improve code readability and mathematical intuition.

### Implementation Constraints

- **No Mathematical Libraries**: You are strictly forbidden from using any third-party or standard mathematical libraries (e.g., numpy, math.h, etc.) unless explicitly stated otherwise.
- **Algorithmic Efficiency**:
  - **Time Complexity**: Calculated relative to the number of executed instructions.
  - **Space Complexity**: Calculated relative to the maximum memory allocated simultaneously.
  - **Input Scaling**: Both complexities must be optimized based on the size of the function’s input.

---

## Technical Theory: Linear Structures

This project implements fundamental linear structures based on formal Abstract Algebra. The core objective is to maintain the mathematical integrity of Vector Spaces and Linear Maps.

### 1. Vector Space (V) over a Field (K)

A Vector Space is an algebraic structure that associates a set of scalars with a set of vectors.

- **The Field (K)**: An algebraic structure where addition, subtraction, multiplication, and division follow the rules of usual arithmetic. Elements of K are called "scalars".
- **The Commutative Group (V)**: An algebraic structure where elements can be added and subtracted. Elements of V are called "vectors".
- **Equivalence to K^n**: In finite dimensions, any vector space V is equivalent to $K^n$. This implies that a vector can always be represented as a list, array, or tuple containing n elements of K.

### 2. Axioms of Scalar Multiplication

Scalar multiplication is an operation that combines elements of K and V to return an element of V. It must satisfy the following formal properties for all $\lambda, \mu \in K$ and $u, v \in V$:

- **Closure**: $\lambda u \in V$.
- **Vector Distributivity**: $\lambda(u + v) = \lambda u + \lambda v$.
- **Scalar Distributivity**: $(\lambda + \mu)u = \lambda u + \mu u$.
- **Associativity**: $(\lambda\mu)u = \lambda(\mu u)$.
- **Identity**: $1_{K}u = u$ (Compatibility with the multiplicative identity of the field).

### 3. Linear Maps and Transformations

A function $f: V \rightarrow W$ is defined as a linear map (or linear transformation) if it satisfies the following conditions:

- **Additivity**: $f(u + v) = f(u) + f(v)$ for all $u, v \in V$.
- **Homogeneity**: $f(\lambda u) = \lambda f(u)$ for all $\lambda \in K, u \in V$.

For finite-dimensional spaces, every linear map is uniquely represented by an $m \times n$ matrix, where $n$ is the dimension of the input space and $m$ is the dimension of the output space.

### 4. Composition and Matrix Multiplication

The composition operator for linear maps serves as the theoretical foundation for matrix multiplication.

- **Successive Transformation**: A composition $(g \circ f)(x) = g(f(x))$ exists if and only if the output type of $f$ matches the input type of $g$.
- **Matrix Product**: Multiplication $AB$ between matrices A and B is defined if and only if the number of columns of A matches the number of rows of B.
- **Efficiency**: While matrix multiplication is associative ($A(BC) = (AB)C$), the computational order affects efficiency. It is optimal to multiply in an order that reduces the size of the intermediate "middle space" to minimize the total number of operations.
