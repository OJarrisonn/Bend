# A cool trick involving HVM's scopeless lambdas is linear qs:

# Qnew : Queue a
Qnew = λx x

# Qadd : a -> Queue a -> Queue a
Qadd = λx λq λk (q λc (c x k))

# Qrem : Queue a -> Pair a (Queue a)
Qrem = λq (q $k λx λxs λp(p x λ$k xs))

Nil = λc λn n
Cons = λh λt λc λn (c h t)

# Output: [1, 2, 3]
main =
  let q = Qnew
  let q = ((Qadd) 1 q)
  let q = ((Qadd) 2 q)
  let q = ((Qadd) 3 q)
  (((Qrem) q) λv0 λq
  (((Qrem) q) λv1 λq
  (((Qrem) q) λv2 λq
  ((Cons) 1 ((Cons) 2 ((Cons) 3 Nil))))))
