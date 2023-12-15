use crate::Graph;

#[derive(Clone)]
pub struct Route{
    pub graph:Graph,
    pub start_nodes:Vec<i64>,
    pub paths:Vec<Vec<Vec<i64>>>,
}

impl Route{
    pub fn new(g: Graph) -> Self{
        Route{
            graph : g,
            start_nodes: Vec::new(),
            paths: Vec::new(),
        }
    }
    
    pub fn shortest_path(&mut self, start:i64) -> Vec<Vec<i64>> {
        let mut visited = vec![false; (self.graph.max+1).try_into().unwrap()];
        let mut costs = vec![0;  (self.graph.max+1).try_into().unwrap()];
        self.start_nodes.push(start);
        let mut paths:Vec<Vec<i64>> = vec![vec![]; self.graph.nodes.len()];

        let mut f_idx = self.graph.find_node_index(start);
        if f_idx == -1{
            return vec![];
        }

        let mut queue = vec![f_idx as usize];
        if f_idx != -1 {
            visited[f_idx as usize] = true;
        }
        costs[f_idx as usize] = 0;
        paths[f_idx as usize].push(f_idx);

        while queue.len() != 0{
            let cur = queue.remove(0) as usize;
            let next_candi = &self.graph.edges[cur];
            for n in next_candi{
                f_idx = self.graph.find_node_index(*n);
                if f_idx != -1 && visited[f_idx as usize] == false{
                    queue.push(f_idx as usize);
                    visited[f_idx as usize] =true;
                    costs[f_idx as usize] = costs[cur] + 1 as i64;
                    paths[f_idx as usize] = paths[cur].clone();
                    paths[f_idx as usize].push(f_idx);
                }

            }
        }
        self.paths.push(paths);
        return self.paths[self.start_nodes.len()-1].clone();
    }
    
    pub fn get_route(&mut self, start:i64, end:i64) -> Vec<i64> {
       
        if self.graph.find_node_index(start) == -1 || self.graph.find_node_index(end) == -1 {
            return vec![];
        }
        for i in 0..self.start_nodes.len(){
            if self.start_nodes[i] == start {
                return self.real_node(self.paths[i][self.graph.find_node_index(end) as usize].clone());
            }
        }
        let result = self.shortest_path(start);
        return self.real_node(result[self.graph.find_node_index(end) as usize].clone());
    }

    fn real_node(&self,path:Vec<i64>) -> Vec<i64>{
        let mut result:Vec<i64> = Vec::new();
        for i in path{
            result.push(self.graph.nodes[i as usize]);
        }
        return result.clone();
    }
    
}

