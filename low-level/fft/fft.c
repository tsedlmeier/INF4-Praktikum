#include <stdio.h>
#include <stdlib.h>
#include <complex.h>

#define M_PI 3.1415926535
#define D 8

typedef float complex cfloat;
typedef int complex cint;

cfloat mypow(cfloat e, int x)
{
  cfloat res = e; 
  for (int i = 0; i < x; i++) {
    res *= e; 
  }
  return res;
}

void fft(cfloat* x, cfloat* y, int N)
{

  if(N==1) {
    y[0] = x[0];
    return;
  }

  cfloat* X_e = (cfloat*)malloc(N * sizeof(cfloat));
  cfloat* X_o = (cfloat*)malloc(N * sizeof(cfloat));
  cfloat* Y_e = (cfloat*)malloc(N * sizeof(cfloat));
  cfloat* Y_o = (cfloat*)malloc(N * sizeof(cfloat));
    
  for (int i = 0; i < N/2; i++) {
    X_e[i] = x[2*i];
    X_o[i] = x[2*i+1];
  }

  fft(X_e, Y_e, N/2);
  fft(X_o, Y_o, N/2);

  cfloat w = cexp(2*M_PI*(-I)/N);
  // cfloat w = cexp(2 * M_PI * (0.0 - 1.0i) / N);
  for (int i = 0; i < N/2; i++) {
    cfloat tw = cpow(w,i);
    // cfloat tw = mypow(w,i);
    y[i]      = Y_e[i] + tw * Y_o[i];
    y[i+N/2]  = Y_e[i] - tw * Y_o[i];
  }

  free(X_e);
  free(X_o);
  free(Y_e);
  free(Y_o);
}

void ifft(cfloat* x, cfloat* y, int N)
{

  if(N==1) {
    y[0] = x[0];
    return;
  }

  cfloat* X_e = (cfloat*)malloc(N * sizeof(cfloat));
  cfloat* X_o = (cfloat*)malloc(N * sizeof(cfloat));
  cfloat* Y_e = (cfloat*)malloc(N * sizeof(cfloat));
  cfloat* Y_o = (cfloat*)malloc(N * sizeof(cfloat));
  
  for (int i = 0; i < N/2; i++) {
    X_e[i] = x[2*i];
    X_o[i] = x[2*i+1];
  }

  fft(X_e, Y_e, N/2);
  fft(X_o, Y_o, N/2);

  cfloat w = cexpf(2*M_PI*(I)/N);
  for (int i = 0; i < N/2; i++) {
    cfloat tw = cpow(w,i);
    // cfloat tw = mypow(w,i);
    y[i]      = Y_e[i] + tw * Y_o[i];
    y[i+N/2]  = Y_e[i] - tw * Y_o[i];
  }

  free(X_e);
  free(X_o);
  free(Y_e);
  free(Y_o);
}

int main ()
{
  cfloat x[D] = {0,1,2,3,4,5,6,7};
  cfloat X[D];
  fft(x, X, D);
  printf("FFT\n");
  for (int i = 0; i<D; i++)
  {
      printf("(%.4f, %.4f i)\n", creal(X[i]), cimag(X[i]));
  }

  ifft(x, X, D);
  printf("IFFT\n");
  for (int i = 0; i<D; i++)
  {
      printf("(%.4f, %.4f i)\n", creal(x[i]), cimag(X[i]));
  }

  
  return 0;
}
