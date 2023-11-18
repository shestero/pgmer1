# pgmer1
MeritRank pgrx HTTP connector

How to run and check:
```
.../pgmer1$ cargo pgrx run
...
psql (15.4)
Type "help" for help.

//// in case you have installed previous version build drop it:
pgmer1=# drop extension pgmer1;
DROP EXTENSION

pgmer1=# create extension pgmer1;
CREATE EXTENSION

//// just to check the extension is loaded:
pgmer1=# select mr_service_url() ;
    mr_service_url     
-----------------------
 http://localhost:8000
(1 row)

//// create an edge:
pgmer1=# select mr_edge('t1','t2',1.0) ;
                mr_edge                
---------------------------------------
 "Added edge t1 -> t2 with weight 1.0"
(1 row)

pgmer1=# select * from mr_scores('t1');
 node | ego |       score        
------+-----+--------------------
 t1   | t1  | 0.5417118093174431
 t2   | t1  | 0.4582881906825569
(2 rows)

pgmer1=# select mr_node_score('t1','t2');
   mr_node_score    
--------------------
 0.4582881906825569
(1 row)
```
