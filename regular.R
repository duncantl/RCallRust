dyn.load("wrapper.so")
.C("R_jane", 3L)

.Call("Rf_bob", 3.12)

