pub mod closures {
    #[macro_export]
    macro_rules! async_closure {
        ( ($( $var:ident ),*), ($( $arg:ident ),*), $closure:expr )
        => {{
            $(let $var = std::sync::Arc::clone(&$var);)*

            move |$($arg),*| {
                $(let $var = std::sync::Arc::clone(&$var);)*

                std::boxed::Box::pin($closure)
            }
        }};
    }

    #[macro_export]
    macro_rules! async_box_closure {
        ( ($( $var:ident ),*), ($( $arg:ident ),*), $closure:expr )
        => {
            std::boxed::Box::new($crate::async_closure!(($($var),*), ($($arg),*), $closure))
        };
    }

    #[macro_export]
    macro_rules! async_arc_closure {
        ( ($( $var:ident ),*), ($( $arg:ident ),*), $closure:expr )
        => {
            std::sync::Arc::new($crate::async_closure!(($($var),*), ($($arg),*), $closure))
        };
    }
}

pub mod types {
    #[macro_export]
    macro_rules! box_future_type {
        ( ( $($arg:ty),* ), $output:ty ) => {
            Box<dyn Fn($($arg),*) -> futures_util::future::BoxFuture<'static, $output> + Send + Sync>
        };
    }

    #[macro_export]
    macro_rules! arc_future_type {
        ( ( $($arg:ty),* ), $output:ty ) => {
            Arc<dyn Fn($($arg),*) -> futures_util::future::BoxFuture<'static, $output> + Send + Sync>
        };
    }
}

#[cfg(test)]
#[allow(unused_allocation)]
pub mod tests {
    use crate::{arc_future_type, async_arc_closure, async_box_closure, box_future_type};
    use std::sync::Arc;

    #[tokio::test]
    async fn box_empty() {
        type ClosureFn = box_future_type!((), bool);

        let closure: ClosureFn = async_box_closure!((), (), async { true });

        assert!(closure().await);
    }

    #[tokio::test]
    async fn arc_empty() {
        type ClosureFn = arc_future_type!((), bool);

        let closure: ClosureFn = async_arc_closure!((), (), async { true });

        assert!(closure().await);
    }

    #[tokio::test]
    async fn arc_clone_empty() {
        type ClosureFn = arc_future_type!((), bool);

        let closure: ClosureFn = async_arc_closure!((), (), async { true });

        let result1: bool = Arc::clone(&closure)().await;
        let result2: bool = Arc::clone(&closure)().await;

        assert!(result1);
        assert!(result2);
    }

    #[tokio::test]
    async fn capture_one() {
        type ClosureFn = box_future_type!((), bool);

        let value = Arc::new(true);
        let closure: ClosureFn = async_box_closure!((value), (), async move { *value });

        assert!(closure().await);
    }

    #[tokio::test]
    async fn capture_two() {
        type ClosureFn = box_future_type!((), bool);

        let a = Arc::new(5);
        let b = Arc::new(5);

        let closure: ClosureFn = async_box_closure!((a, b), (), async move { (*a + *b) == 10 });

        assert!(closure().await);
    }

    #[tokio::test]
    async fn arg_one() {
        type ClosureFn = box_future_type!((i32), bool);

        let closure: ClosureFn = async_box_closure!((), (value), async move { value == 10 });

        assert_eq!(closure(10).await, true);
        assert_eq!(closure(5).await, false);
    }

    #[tokio::test]
    async fn arg_two() {
        type ClosureFn = box_future_type!((i32, i32), bool);

        let closure: ClosureFn = async_box_closure!((), (a, b), async move { (a + b) == 10 });

        assert_eq!(closure(5, 5).await, true);
        assert_eq!(closure(10, 10).await, false);
    }

    #[tokio::test]
    async fn total() {
        type ClosureFn = box_future_type!((i32, i32), i32);

        let arc_a = Arc::new(1);
        let arc_b = Arc::new(1);

        let closure: ClosureFn = async_box_closure!((arc_a, arc_b), (a, b), async move {
            (*arc_a + *arc_b) * (a + b)
        });

        assert_eq!(closure(2, 3).await, 10);
        assert_eq!(closure(1, 1).await, 4);
    }
}
