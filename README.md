
This is a very simple example of calling Rust routines from R.
This uses the Rffi package to call the routines directly without
the need for a wrapper routine. We could use Dan Adler's rdyncall package
also.

``` shell
make
```

In R
``` R
source("call.R", verbose = TRUE)
```