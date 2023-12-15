# Best Route Search Program

## 1. Project Information
    > This project is a program that generates, for each row, a node for column1 and column2. Then, it creates a graph shows the direct connection from column1 to column2. The user inputs the starting point and the ending point, and this graph finds the shortest path through BFS(breadth first search)
## 2. Functions
- graph module
    - new(): This function is for resetting the graph, so it resets the lists of nodes and edges.
    - print_graph(&self): This function prints the graph
    - print_edge(&self, node:i64): This function returns the edge of the node that corresponds to the value passed in to the parameter node from the graph. 
    - add_node(&mut self,loc:i64): Adds the value passed into the loc parameter as a node
    - add_edge(&mut self,start_loc:i64,end_loc:i64): connects an edge from the start_loc paramter to the end_loc
- route module
    - new(g: Graph): Initialize the received graph and initialize the variable that store the route of the corresponding node. 
    - shortest_path(&mut self, start:i64): It calculates and stores the shortest path from the start parameter to the rest of the other nodes and returns it.
    - get_route(&mut self, start:i64, end:i64): Function that returns the shortest path from the start parameter to the end paramter 
    - real_node(&self,path:Vec<i64>): receives a list containing indices of a path as input (named 'path'), converts them into actual nodes, and returns the result
- main 
    - read_file(path: &str): reads and returns the file of the path passed in the path parameter.
    - select_three(mut paths:Route): takes a variable passed to 'paths' along with the start and end points, and outputs the path 
    - select_second(paths:Route): outputs the edges of the input points in the graph passed to 'paths'
    - main(): opens the euroroad.csv file and creates a graph and a module that cacluates the route, and outputs the content based on the value selected by the user. 

     
## 3. How to run it 
  ### 1.terminal cargo run    
  #### 1) If you want to see a part of the graph in the first screen, enter 1. If you want to search for a edge list for a particular vertex, enter 2. If you want to search for a route, enter 3, and if you want to end the program, enter 4. 

```
  1. Graph output
  2. Edge search
  3. Find route
  4. End
  select
  <input space>
```

#### 2.1)Entering 1 at option 1_graph will be printed 
```
    edges : [1169]
    nodes : 1169
    edges : [1170]
    nodes : 1172
    edges : []
    nodes : 1173
    edges : [1174]
    nodes : 1174
    edges : []
    ------------------------
```

#### 2.2) Entering 1 at 2_If you enter a node, you can search for the edges according to that node
```
    1. Graph output
    2. Edge search
    3. Find route
    4. End
    select
    2
    Enter Node: 
    <Enter the node that you want to see>

```

#### 2.3) entering 1 at 3_ Enter the starting route and the ending route, and you can see the route
```
    1. Graph output
    2. Edge search
    3. Find route
    4. End
    select
    3
    Enter Start Node: 
    <this is where you enter the Starting route>
    Enter End Node: 
    <this is where you enter the ending route>
```

#### 2.4) Entering 1 at 4_ Ends the program

### 2.Input cargo test in the terminal
> execution of the results of test graph

## 4. Output
### Selected screen 
 ```
 1. Graph output
 2. Edge search
 3. Find route
 4. End
 select
 ```
### If you entered 1 on the first screen_ graph is printed
```
nodes : 1169
edges : [1170]
nodes : 1172
edges : []
nodes : 1173
edges : [1174]
nodes : 1174
edges : []
```
### If you entered 2 on the first screen_If you enter a node, you can search for the edges according to that node
 ```
Enter Node: 
4
------------------------

node 4 edges: [5, 855]
------------------------
 ```
 ### if you entered 3 on the first screen_ enter the starting and ending paths, and the program will then output the path.
 ```
Enter Start Node: 
1
Enter End Node: 
855
Start 1 Node to End 855 Node Shortest Path: 
[1, 2, 3, 4, 855]
------------------------
```
### If you entered 4 on the first screen _ It prints the program ending message
 ```
select
4
-----------Program End-----------
```


 