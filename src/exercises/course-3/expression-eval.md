# Exercise: Expression Evaluation

Let's write a simple recursive evaluator for arithmetic expressions.

An example of a small arithmetic expression could be `10 + 20`, which evaluates
to `30`. We can represent the expression as a tree:

```
        +
       / \
      10  20
```

A bigger and more complex expression would be `(10 * 9) + ((3 - 4) * 5)`, which
evaluates to `85`.

In code, we will represent the tree with two types:

```rust,editable
/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// A literal value
    Value(i64),
}
```

The `Box` type here is a smart pointer. An expression can be "boxed" with
`Box::new` as seen in the tests. To evaluate a boxed expression, use the deref
operator (`*`) to "unbox" it: `eval(*boxed_expr)`.

Implement `eval`. The final product should pass the tests.

```rust,editable
/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// A literal value
    Value(i64),
}

fn eval(e: Expression) -> i64 {
    todo!()
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), 19);
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        30
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        85
    );
}

fn main() {
    let expr = Expression::Op {
        op: Operation::Add,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(20)),
    };
    println!("{expr:?} = {}", eval(expr));
}
```

<details>
<summary>Hints</summary>

- Use `match` to destructure the `Expression` enum.
- For `Expression::Value(v)`, simply return `v`.
- For `Expression::Op`, recursively evaluate both sides and apply the operation.
- Remember to dereference the boxed expressions with `*`.

</details>
