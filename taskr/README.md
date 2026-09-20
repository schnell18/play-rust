# Introduction

Minimalist task manager built with Rust Axum framework.

# Tests

## List all tasks
  
  curl http://localhost:4000/tasks
  
## Get one task
  
  curl http://localhost:4000/tasks/1
  curl http://localhost:4000/tasks/2
  curl http://localhost:4000/tasks/3
  
## Create one task
  
  curl -H 'Content-Type: application/json' http://localhost:4000/tasks -d'{"title": "ship true part two"}'
  
## Update one task
  
  curl -H 'Content-Type: application/json' -XPATCH http://localhost:4000/tasks/4 -d'{"title": "ship true part II", "done": true}'
  
## Delete one task
  
  curl -XDELETE http://localhost:4000/tasks/1
  curl -vv http://localhost:4000/tasks/1
  curl -vv -XDELETE http://localhost:4000/tasks/5
  curl http://localhost:4000/tasks
