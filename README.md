# pgmer1
MeritRank pgrx HTTP connector

How to run and check:
1. _ ... create edge: nodes n1 and n2 with weight 1 through API ... _
2. run:
```
.../pgmer1$ RUST_BACKTRACE=full cargo pgrx run
...
psql (15.4)
Type "help" for help.

pgmer1=# create extension pgmer1;
CREATE EXTENSION

pgmer1=# select mr_service_url() ;
    mr_service_url     
-----------------------
 http://localhost:8000
(1 row)

pgmer1=# select * from mr_scores('n1');
 node | ego |        score        
------+-----+---------------------
 n1   | n1  |  0.5402193290475933
 n2   | n1  | 0.45978067095240666
(2 rows)

pgmer1=# select * from mr_node_score('n1','n2');
    mr_node_score    
---------------------
 0.45978067095240666
(1 row)
```
