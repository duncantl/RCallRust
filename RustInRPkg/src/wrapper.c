#if 0
int jane(int);

void
R_jane(int *val)
{
   val[0] =  jane(val[0]);
}


#include <Rdefines.h>

double bob(double);

SEXP
Rf_bob(SEXP r_val)
{
    double val;
    val = bob(REAL(r_val)[0]);
    return(ScalarReal(val));
}

#endif

extern void process();

void
R_process()
{
    process();
}
