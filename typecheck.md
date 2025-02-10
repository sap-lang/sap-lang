// pure function
@@sig a = \a b c -> a
a = \a _ _ -> a

// cofunction
@@sig c = \a b -> c <- b
c = \a b -> {
    c =<= b
    c
}

@@type @template a b \
s = {
    a: a
    b: b
}
