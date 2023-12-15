#[derive(Clone)]
pub struct Graph{
    pub nodes:Vec<i64>,
    pub edges:Vec<Vec<i64>>,
    pub max: i64,
}

impl Graph{
    pub fn new() -> Self{
        Graph{
            nodes: Vec::new(),
            edges: Vec::new(),
            max: -1,
        }
    }

    pub fn print_graph(&self){
        println!("\n\n------Current Graph------");
        for i in 0..self.nodes.len(){
            println!("nodes : {:?}",self.nodes[i]);
            println!("edges : {:?}", self.edges[i]);
        }
        println!("------------------------\n\n");
    }

    pub fn print_edge(&self, node:i64){
        if self.find_node_index(node) != -1{
            println!("------------------------\n");
            println!("node {} edges: {:?}",node,self.edges[self.find_node_index(node) as usize]);
            println!("------------------------\n\n");

        }
    }

    pub fn add_node(&mut self,loc:i64){
        if self.find_node_index(loc) == -1{
            self.nodes.push(loc);
            self.edges.push(vec![]);
            if self.max < loc{
                self.max = loc;
            }
        }
    }
    pub fn add_edge(&mut self,start_loc:i64,end_loc:i64){
        for i in 0..self.nodes.len(){
            if self.nodes[i]== start_loc{
                for j in 0..self.edges[i].len(){
                    if self.edges[i][j] == end_loc{
                        return;
                    }
                }
                self.edges[i].push(end_loc);
                if self.find_node_index(end_loc) == -1{
                    self.add_node(end_loc);
                }
                return;
            }
            
        }
        self.nodes.push(start_loc);
        self.edges.push(vec![end_loc]);

        if self.find_node_index(end_loc) == -1{
            self.add_node(end_loc);
        }
    }

    pub fn find_node_index(&self,loc:i64) -> i64{
        if loc < 0 {
            return -1;
        }
        for i in 0..self.nodes.len(){
            if self.nodes[i] == loc {
                return i as i64;
            }
        }
        return -1;
    }
     
    
}