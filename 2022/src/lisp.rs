#[macro_export]
macro_rules! lisp {
    {
        $first:tt $($more:tt)+
    } => {
        {
            lisp!{$first};
            $(lisp!{$more})+
        }
    };
    {
        (do $($body:tt)+)
    } => {
        {
            $(lisp!{$body})+
        }
    };
    {
        $atom:literal
    } => { $atom };
    {
        $bind:ident
    } => { $bind };
    {
        (let (
            $($bind:ident $value:tt)*
        ) $($body:tt)+)
    } => {
        {
            $(
                let $bind = lisp!{$value};
            )*
            lisp!{ $($body)* }
        }
    };
    {
        (if $test:tt $on_true:tt $($on_false: tt)?)
    } => {
        if lisp!{$test} {
            lisp!{$on_true}
        } $(else {
            lisp!{$on_false}
        })?
    };
    {
        (cell $value:tt)
    } => {
        std::cell::Cell::new(lisp!{$value})
    };
    {
        (slurp-iter $path:tt)
    } => {
        $crate::line::FileLineIterator::new($path)
    };
    {
        (for-each ($var:ident $expr:tt) $($body:tt)+)
    } => {
        for $var in lisp!{$expr} {
            lisp!{$($body)+}
        }
    };
    {
        (< $lhs:tt $rhs:tt)
    } => {
        lisp!{$lhs} < lisp!{$rhs}
    };
    {
        (+ $lhs:tt $rhs:tt)
    } => {
        lisp!{$lhs} + lisp!{$rhs}
    };
    {
        (empty? $expr:tt)
    } => {
        lisp!{$expr}.is_empty()
    };
    {
        (deref $cell:tt)
    } => {
        lisp!{$cell}.get()
    };
    {
        (reset! $cell:tt $value:tt)
    } => {
        lisp!{$cell}.set(lisp!{$value})
    };
    {
        (swap! $cell:tt $op:tt $($args:tt)*)
    } => {
        lisp!{
            (let (cell (ref $cell))
                (reset! cell ($op (deref cell) $($args)*))
            )
        }
    };
    {
        (ref $value:tt)
    } => {
        &lisp!{$value}
    };
    {
        (parse ($ty:ty) $str:tt)
    } => {
        lisp!{$str}.parse::<$ty>().unwrap()
    };
}
