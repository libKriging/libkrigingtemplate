library(libkrigingtemplate)
library(testthat)

nrows <- 20
ncols <- 10
X <- matrix(rnorm(nrows*ncols),nrow=nrows)
y <- matrix(runif(nrows))
rl <- LinearRegression$fit(y, X)
px = rl$predict(X)