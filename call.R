dyn.load("librustRoutines.dylib")

library(Rffi)
f = prepCIF(sint32Type, list(sint32Type))
callCIF(f, "jane", 4L)


# Not behaving itself
f = prepCIF(doubleType, list(doubleType))
callCIF(f, "bob", 4.12)



