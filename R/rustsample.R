#' An example package demonstrating the use of the extendr crate from R.
#'
#' An example package demonstrating the use of the extendr crate from R with an alias sampler.
#' @name rustsample
#' @docType package
#' @useDynLib rustsample, .registration = TRUE
NULL


#' Call an alias sampler built in Rust
#'
#' @description Call an alias sampler built in Rust.
#'
#' @param x either a vector of two or more elements from which to choose, or a positive integer.
#' @param size a non-negative integer giving the number of items to choose.
#' @param replace should sampling be with replacement? Defaults to `TRUE`. See details.
#' @param prob a vector of probability weights for obtaining the elements of the vector being sampled.
#'
#' @details
#'   This function generally has the same behavior as \code{\link[sample]{base}} with the following exception:
#'
#'   Only sampling with replacement is supported. If `replace = FALSE`, you will receive an error.
#'   The `replace` argument is included only for symmetry with arguments in R's base \code{\link[sample]{base}}.
#'
#' @export
rsample <- function(x, size, replace = TRUE, prob = NULL) {

  # check inputs
  if (length(x) == 1L && is.numeric(x) && is.finite(x) && x >= 1) {
    size <- x

    x <- 1:x
  }

  if (! replace) {
    stop("Only sampling with replacement is supported.")
  }

  if (missing(size)) {
    size <- length(x)
  }

  if (is.null(prob)) {
    prob <- rep(1, length(x))
  }

  if (any(prob < 0)) {
    stop("Negative probabilities not allowed.")
  }

  # sample indices, zero indexed
  indices <- .Call("wrap__rsample", prob, size)

  # return sampled values of x
  result <- x[(indices + 1)]

  result

}
