## Borrowing (immutably) { data-transition="fade-out" }

```rust
#[test]
fn show_some_borrows() {

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    let b1 = &v1;
    let b2 = &v2;
    foo(b1);
    foo(b2);

}
```

```rust
fn foo<'a>(v: &'a Vec<i32>) { println!("v[1]: {}", v[1]); }
```

`&v1` and `&v2` are *borrowing* `v1` and `v2`.

## Scopes and Lifetimes { data-transition="fade-in" }

```rust
#[test]
fn show_some_lifetimes() {

    let v1 = vec![1, 2, 3]; //                 +
    let v2 = vec![4, 5, 6]; //            +    |
                            //            |    |
    let b1 = &v1;           //       +    |    |
    let b2 = &v2;           //  +    |    |    |
    foo(b1);                //  |    |    |    |  
    foo(b2);                // 'b2  'b1  'v2  'v1
                            //  |    |    |    | 
}                           //  +    +    +    +
```

``` {.rust}
fn foo<'a>(v: &'a Vec<i32>) { println!("v[1]: {}", v[1]); }
```

Each borrow selects "appropriate" lifetime `'a`.

## Borrow Checking Prevents Errors { data-transition="slide-in fade-out" }

``` {.rust .compile_error}
fn borrow_checking_prevents_errors() {

    let v1 = vec![1, 2, 3];      //        +
                                 //        |
    let b1 = &v1;                //  +    'v1
                                 //  |     |
    consume(v1);                 // 'b1   (moved)
    foo(b1);                     //  |
}                                //  +
```

```{.rust}
    fn foo<'a>(v: &'a Vec<i32>) { println!("v[1]: {}", v[1]); }
    fn consume(v: Vec<i32>) { }
```

``` {.fragment}
error: cannot move out of `v1` because it is borrowed
    consume(v1);
            ^~
note: borrow of `v1` occurs here
    let b1 = &v1;
              ^~
```

`foo(b1)` attempts an indirect read of `v1`

## Lifetimes and Lexical Scope { data-transition="fade-in" }

``` {.rust .compile_error}
fn borrow_checking_may_seem_simple_minded() {

    let v1 = vec![1, 2, 3];      //        +
                                 //        |
    let b1 = &v1;                //  +    'v1
                                 //  |     |
    consume(v1);                 // 'b1   (moved)
    // (no call to read)         //  |
}                                //  +
```

```{.rust}
    fn foo<'a>(v: &'a Vec<i32>) { println!("v[1]: {}", v[1]); }
    fn consume(v: Vec<i32>) { }
```

```
error: cannot move out of `v1` because it is borrowed
    consume(v1);
            ^~
note: borrow of `v1` occurs here
    let b1 = &v1;
              ^~
```

(artifact of lexical-scope based implementation)

## Lifetime nesting  { data-transition="slide" }

```rust
#[test]
fn lifetime_nesting() {
    let v1 = vec![1, 2, 3];         //                      +
    let b1;                         //                      |
    {                               //                      |
        let v2 = vec![4, 5, 6];     //                 +    |
        {                           //                 |    |
            let b2 = &v2;           //  +              |    |
            let v3 = vec![7,8,9];   //  |         +   'v2  'v1
            b1 = &v1;               // 'b2   +   'v3   |    |
            foo(b2);                //  |    |    |    |    |
        }                           //  +   'b1   +    |    |
    }                               //       |         +    |
    foo(b1);                        //       |              |
}                                   //       +              +
```

``` {.rust}
fn foo<'a>(v: &'a Vec<i32>) { println!("v[1]: {}", v[1]); }
```

borrow `&v2` is for at least `'b2`, but at most `'v2`

## Lexical scopes, but Nontrivial { data-transition="fade-out" }

```rust
#[test]
fn copying_can_extend_a_borrows_lifetime() {
    fn foo<'a>(v: &'a Vec<i32>) {
        println!("v[1]: {}", v[1]);
    }
    let v1 = vec![1, 2, 3]; //         +
    let b2 = {              //         |
        let b1 = &v1;       //  +      |
        //       ^~~        //  |      |
        foo(b1);            // 'b1     |
        b1                  //  |     'v1
    };                      //  +  +   |
                            //     |   |
    foo(b2);                //    'b2  |
                            //     |   |
}                           //     +   +
```

(How long does the borrow `&v1` last? Does `'b1` suffice?)

## Lexical Scopes: But Nontrivial { data-transition="fade-in" }

``` {.rust}
#[test]
fn copying_can_extend_a_borrows_lifetime() {
    fn foo<'a>(v: &'a Vec<i32>) {
        println!("v[1]: {}", v[1]);
    }
    let v1 = vec![1, 2, 3]; //         +
    let b2: &'z Vec<i32> = {//         |
        let b1 = &'z v1;    //  +      |
        //       ^~~~~~     //  |      |
        foo(b1);//  |       // 'b1     |
        b1      // (caveat) //  |     'v1 == 'z
    };                      //  +  +   |
                            //     |   |
    foo(b2);                //    'b2  |
                            //     |   |
}                           //     +   +
```

`'b1` too short (caveat: above is not legal Rust)
