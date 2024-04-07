# Thoughts


Mutex does not unlock in free_intercept since print uses malloc and malloc uses malloc_intercept which tries to access the Mutex

