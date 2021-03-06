use std::fmt;
mod free_list;
use free_list::FreeList;


// From answer here: https://stackoverflow.com/questions/41946007/efficient-and-well-explained-implementation-of-a-quadtree-for-2d-collision-det


//QuadElt is stored once, and is referred to by QuadEltNode
#[derive(Debug)]
struct QuadElt {

    pub id: i32,

    pub rect: QuadRect
}

#[derive(Debug)]
struct QuadEltNode {
    //next node -1 is end of list
    pub next : i32,

    // index of element
    pub element: i32
}


impl QuadEltNode {

    pub fn insert(id: i32, node: &mut QuadNode, element_nodes: &mut FreeList<QuadEltNode>) {

        //println!("Inserting node for element with id: {:?}", id);
        let elm_node_index = element_nodes.insert(QuadEltNode {
            next: -1,
            element: id
        });

        node.count += 1;

        let mut last_node_index = node.first_child;

        if last_node_index == -1  {
            node.first_child = elm_node_index;
        }
        else {
            //println!("last node: {:?}", last_node_index);
            while element_nodes[last_node_index].element.next != -1 {

                last_node_index = element_nodes[last_node_index].element.next;
            }

            element_nodes[last_node_index].element.next = elm_node_index;

            if last_node_index > 100{
                //panic!();
            }
        }

    }
}

#[derive(Debug)]
struct QuadNode {

    // child are stored continiues
    // child0 (TL) = first_child
    // child1 (TR) = first_child + 1
    // child2 (BL) = first_child + 2
    // child3 (BR) = first_child + 3
    // if count is > 0 then its a leaf and first_child referres to element_nodes (QuadEltNode)
    pub first_child: i32,


    pub count: i32
}

impl QuadNode {

    pub fn leaf() -> Self {
        QuadNode {
            first_child: -1,
            count: 0
        }
    }
}


pub enum Query {
    Point(QuadPoint),
    Rect(QuadRect)
}

impl Query {
    pub fn point(x: i32, y: i32) -> Self {
        Query::Point(QuadPoint { x, y })
    }

    pub fn rect(rect: QuadRect) -> Self {
        Query::Rect(rect)
    }
}

#[derive(Debug, Clone)]
pub struct QuadRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32
}

#[derive(Debug)]
pub struct QuadPoint {
    pub x: i32,
    pub y: i32
}

impl QuadRect {

    pub fn new(p1: QuadPoint, p2: QuadPoint) -> Self {

        QuadRect {
            left: i32::min(p1.x, p2.x),
            right: i32::max(p1.x, p2.x),
            top:  i32::max(p1.y, p2.y),
            bottom: i32::min(p1.y, p2.y),
        }
    }

    fn location_quad(&self, i: usize) -> QuadRect {

        let node_middle_x = (self.right - self.left) / 2 + self.left;
        let node_middle_y = (self.top - self.bottom) / 2 + self.bottom;
        let middle_point = QuadPoint{x: node_middle_x, y: node_middle_y};
        return match i {
            // TL
            0 => QuadRect::new(middle_point, QuadPoint{x: self.left, y: self.top}),
            // TR
            1 => QuadRect::new(middle_point, QuadPoint{x: self.right, y: self.top}),
            // BL
            2 => QuadRect::new(middle_point, QuadPoint{x: self.left, y: self.bottom}),
            // BR
            3 => QuadRect::new(middle_point, QuadPoint{x: self.right, y: self.bottom}),
            _ => panic!("Location {} is not a valid location. Valid locations are: 0,1,2,3", i),
        }
    }


    fn point_quad_locations(node_rect: &QuadRect, point: &QuadPoint) -> [bool; 4] {

        // return bool for TL, TR, BL, BR

        let node_middle_x = (node_rect.right - node_rect.left) / 2 + node_rect.left;
        let node_middle_y = (node_rect.top - node_rect.bottom) / 2 + node_rect.bottom;

        // check is it inside on X and Y
        let tl = point.x <= node_middle_x &&
            point.x >= node_rect.left &&
            point.y >= node_middle_y &&
            point.y <= node_rect.top;


        let tr = point.x >= node_middle_x &&
            point.x <= node_rect.right &&
            point.y >= node_middle_y &&
            point.y <= node_rect.top;


        let bl = point.x <= node_middle_x &&
            point.x >= node_rect.left &&
            point.y <= node_middle_y &&
            point.y >= node_rect.bottom;


        let br = point.x >= node_middle_x &&
            point.x <= node_rect.right &&
            point.y <= node_middle_y &&
            point.y >= node_rect.bottom;


        [tl, tr, bl, br]

    }

    fn element_quad_locations(node_rect: &QuadRect, element_rect: &QuadRect) -> [bool; 4] {

        // return bool for TL, TR, BL, BR

        let node_middle_x = (node_rect.right - node_rect.left) / 2 + node_rect.left;
        let node_middle_y = (node_rect.top - node_rect.bottom) / 2 + node_rect.bottom;


        // check is it inside on X and Y
        let tl = element_rect.left <= node_middle_x &&
            element_rect.right >= node_rect.left &&
            element_rect.top >= node_middle_y &&
            element_rect.bottom <= node_rect.top;


        let tr = element_rect.right >= node_middle_x &&
            element_rect.left <= node_rect.right &&
            element_rect.top >= node_middle_y &&
            element_rect.bottom <= node_rect.top;


        let bl = element_rect.left <= node_middle_x &&
            element_rect.right >= node_rect.left &&
            element_rect.bottom <= node_middle_y &&
            element_rect.top >= node_rect.bottom;


        let br = element_rect.right >= node_middle_x &&
            element_rect.left <= node_rect.right &&
            element_rect.bottom <= node_middle_y &&
            element_rect.top >= node_rect.bottom;


        [tl, tr, bl, br]

    }
}



pub struct QuadTree<T> {

    // All elements in quadTree
    elements: FreeList<QuadElt>,

    // All elementNodes in quadTree
    element_nodes: FreeList<QuadEltNode>,


    // All nodes in quadTree
    // First node is the root
    nodes: Vec::<QuadNode>,

    data: Vec::<T>,

    // Rect for the root
    // All sub rects are computed on the fly in integers
    root_rect: QuadRect,

    free_node: i32,

    max_depth: i32,
    nodes_per_cell: i32

}


impl<'a, T> QuadTree<T> {

    pub fn new(rect: QuadRect) -> Self {

        let mut nodes = Vec::new();

        nodes.push(QuadNode {
            first_child: -1,
            count: 0,
        });

        QuadTree {
            elements: FreeList::new(),
            element_nodes: FreeList::new(),
            nodes,
            data: Vec::new(),
            root_rect: rect,
            free_node: 0,
            max_depth: 10,
            nodes_per_cell: 6
        }
    }

    pub fn insert(&mut self, element: T, element_rect: QuadRect) {

        //println!("inserting {:?}", element_rect);
        // check if we can insert into root

        self.data.push(element);
        let data_id = (self.data.len() - 1) as i32;



        let element_id = self.elements.insert(QuadElt {
            id: data_id,
            rect: element_rect.clone()
        });

        //println!("Inserting node for element with id: {:?}", element_id);
        let rect = self.root_rect.clone();
        self.insert_elm(element_id, 0, &element_rect, &rect, 0);
    }


    fn insert_elm(&mut self, element_id: i32,  node_index: usize, element_rect: &QuadRect, node_rect: &QuadRect, depth: i32) {

        //println!("node_index = {} depth = {} {:?}", node_index, depth, self.nodes[node_index]);

        // Check if leaf
        if self.nodes[node_index].count > -1 {

            // Check if we can just insert into this node
            if self.nodes[node_index].count < self.nodes_per_cell  || depth >= self.max_depth {
                QuadEltNode::insert(element_id, &mut self.nodes[node_index], &mut self.element_nodes);
            }
            // make this into not a leaf, but a branch
            else {

                //println!("SPLITTING NODE_INDEX: {}", node_index);

                self.split(node_index, node_rect);

                self.nodes[node_index].count = -1;

                self.insert_into_branch(element_id, node_index, element_rect, node_rect, depth);
            }

        }
        else {
            self.insert_into_branch(element_id, node_index, element_rect, node_rect, depth);
        }
    }


    fn insert_into_branch(&mut self, element_id: i32, node_index: usize, element_rect: &QuadRect, node_rect: &QuadRect, depth: i32) {

        // We are at a branch
        // check which children it should be se into
        let locations = QuadRect::element_quad_locations(node_rect, element_rect);


        for i in 0..4 {
            if locations[i] {
                //println!("Inserting {:?} in to {}", element_rect, i);

                let new_node_index = (self.nodes[node_index].first_child as usize) + i;

                let new_rect = node_rect.location_quad(i);

                self.insert_elm(element_id, new_node_index, element_rect, &new_rect, depth + 1);
            }
        }
    }



    fn split(&mut self, node_index: usize, node_rect: &QuadRect) {
        //println!("Making leaf into branch {:?}", node_index);

        self.nodes.push(QuadNode::leaf());

        let new_first_child = self.nodes.len() - 1;

        self.nodes.push(QuadNode::leaf());
        self.nodes.push(QuadNode::leaf());
        self.nodes.push(QuadNode::leaf());


        let mut next_child = self.nodes[node_index].first_child;

        while next_child != -1 {

            //println!("Reallocate element {:?}", self.element_nodes[next_child].element);
            //println!("Original child count {}", self.nodes[node_index].count );
            let reallocated_id = self.element_nodes[next_child].element.element;

            let new_next_child = self.element_nodes[next_child].element.next;

            self.element_nodes.erase(next_child);

            let child_rect = &self.elements[reallocated_id].element.rect;
            let locations = QuadRect::element_quad_locations(node_rect, child_rect);

            for i in 0..4 {
                if locations[i] {
                    //println!("SPLIT INSERT INDEX: base_index={} i={}  new_index={}  {:?}", new_first_child, i, new_first_child + i, self.nodes[new_first_child + i]);
                    QuadEltNode::insert(reallocated_id, &mut self.nodes[new_first_child + i], &mut self.element_nodes);
                }
            }

            next_child = new_next_child;

        }



        // set first child as the first quadnode TL
        // and set count to -1 to indicate it is a branch
        self.nodes[node_index].first_child = new_first_child as i32;
        self.nodes[node_index].count = -1;
    }



    pub fn query(&self, query: &Query) -> Vec::<&T> {

        let root_rect = self.root_rect.clone();

        let mut element_ids = std::collections::HashSet::new();
        self.query_node_box(0, &root_rect, query, &mut element_ids);

        let mut res = Vec::new();

        for index in element_ids.into_iter() {
            res.push(&self.data[index as usize]);
        }
        res

    }


    fn query_node_box(&self, node_index: usize, node_rect: &QuadRect, query: &Query, data_vec: &mut std::collections::HashSet::<i32>) {
        // leaf, return  all elements
        if self.nodes[node_index].count > -1 {

            let mut child_index = self.nodes[node_index].first_child;

            while child_index != -1 {
                data_vec.insert(self.elements[self.element_nodes[child_index].element.element].element.id);

                child_index = self.element_nodes[child_index].element.next;

            }
        }
        else {
            self.query_branch(node_index, node_rect, query, data_vec);
        }
    }


    fn query_branch(&self, node_index: usize, node_rect: &QuadRect, query: &Query, data_vec: &mut std::collections::HashSet::<i32>) {

        let locations = match query {
            Query::Point(p) => QuadRect::point_quad_locations(node_rect, p),
            Query::Rect(b) => QuadRect::element_quad_locations(node_rect, b )
        };

        for i in 0..4 {
            if locations[i] {
                // point is inside this rect
                self.query_node_box((self.nodes[node_index].first_child as usize) + i, &node_rect.location_quad(i), query, data_vec);
            }
        }
    }



    fn print(&self) -> String {
        self.print_node(0, 0)
    }

    fn print_node(&self, node_index: usize, indent: usize) -> String {


        if self.nodes[node_index].count >= 0 {
            // leaf

            if self.nodes[node_index].count > 0 {

                let mut child_index = self.nodes[node_index].first_child;

                let mut res = "".to_string();
                while child_index != -1 {
                    let elm_node = &self.element_nodes[child_index].element;
                    res += &format!(" element: {}, node: {} | ", elm_node.element, elm_node.next);
                    child_index = elm_node.next;
                }





                return format!("\n{:indent$}-{}", "", res, indent=indent);
            }
            else {
                return format!("\n{:indent$}-Empty", "", indent=indent );
            }
        }
        else {
            // branch

            let first_index = self.nodes[node_index].first_child as usize;

            let mut res = format!("\n{:indent$}Branch", "", indent=indent );


            res += &self.print_node(first_index, indent + 4);
            res += &self.print_node(first_index + 1, indent + 4);
            res += &self.print_node(first_index + 2, indent + 4);
            res += &self.print_node(first_index + 3, indent + 4);

            return res;

        }

    }
}


impl<T> fmt::Display for QuadTree<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

impl<T> fmt::Debug for QuadTree<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }

}



#[cfg(test)]
mod test {

    use super::*;

    


    #[test]
    fn node_locations_all() {

        let node_rect = QuadRect::new(QuadPoint {x: -128, y: -128}, QuadPoint { x: 128, y: 128} );

        let element_rect = QuadRect::new(QuadPoint {x: -10, y: -10}, QuadPoint { x: 20, y: 20} );

        let locations = QuadRect::element_quad_locations(&node_rect, &element_rect);

        println!("{:?}", locations);

        assert!(locations[0] && locations[1] && locations[2] && locations[3]);

    }



    #[test]
    fn node_locations_tl() {

        let node_rect = QuadRect::new(QuadPoint {x: -128, y: -128}, QuadPoint { x: 128, y: 128} );

        let element_rect = QuadRect::new(QuadPoint {x: -10, y: 10}, QuadPoint { x: -20, y: 20} );

        let locations = QuadRect::element_quad_locations(&node_rect, &element_rect);

        println!("{:?}", locations);

        assert!(locations[0] && !locations[1] && !locations[2] && !locations[3]);

    }


    #[test]
    fn node_locations_tr() {

        let node_rect = QuadRect::new(QuadPoint {x: -128, y: -128}, QuadPoint { x: 128, y: 128} );

        let element_rect = QuadRect::new(QuadPoint {x: 10, y: 10}, QuadPoint { x: 20, y: 20} );

        let locations = QuadRect::element_quad_locations(&node_rect, &element_rect);

        println!("{:?}", locations);

        assert!(!locations[0] && locations[1] && !locations[2] && !locations[3]);
    }


    #[test]
    fn node_locations_bl() {

        let node_rect = QuadRect::new(QuadPoint {x: -128, y: -128}, QuadPoint { x: 128, y: 128} );

        let element_rect = QuadRect::new(QuadPoint {x: -10, y: -10}, QuadPoint { x: -20, y: -20} );

        let locations = QuadRect::element_quad_locations(&node_rect, &element_rect);

        println!("{:?}", locations);

        assert!(!locations[0] && !locations[1] && locations[2] && !locations[3]);
    }



    #[test]
    fn node_locations_br() {

        let node_rect = QuadRect::new(QuadPoint {x: -128, y: -128}, QuadPoint { x: 128, y: 128} );

        let element_rect = QuadRect::new(QuadPoint {x: 10, y: -10}, QuadPoint { x: 20, y: -20} );

        let locations = QuadRect::element_quad_locations(&node_rect, &element_rect);

        println!("{:?}", locations);

        assert!(!locations[0] && !locations[1] && !locations[2] && locations[3]);
    }


    #[test]
    fn insert_2_elm() {

        let rect = QuadRect::new(QuadPoint {x: -128, y: -128}, QuadPoint { x: 128, y: 128} );

        let mut qt = QuadTree::<f32>::new(rect);


        let elm1_id = 1.0;
        let elm1_rect = QuadRect::new(QuadPoint {x: 10, y: 10}, QuadPoint { x: 20, y: 20} );
        qt.insert(elm1_id, elm1_rect);

        let elm2_id = 2.0;
        let elm2_rect = QuadRect::new(QuadPoint {x: -10, y: -10}, QuadPoint { x: -20, y: -20} );
        qt.insert(elm2_id, elm2_rect);


        let mut points0 = qt.query(&Query::point(15,15));

        points0.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert!(*points0[0] == 1.0);
        assert!(*points0[1] == 2.0);
        assert!(points0.len() == 2);


        let mut points1 = qt.query(&Query::point(-1,-1));
        points1.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!(*points1[0] == 1.0);
        assert!(*points1[1] == 2.0);
        assert!(points1.len() == 2);

    }

    #[test]
    fn insert_3_elm() {

        let rect = QuadRect::new(QuadPoint {x: -128, y: -128}, QuadPoint { x: 128, y: 128} );

        let mut qt = QuadTree::<f32>::new(rect);

        let elm0_id = 0.0;
        let elm0_rect = QuadRect::new(QuadPoint {x: 10, y: 10}, QuadPoint { x: 20, y: 20} );
        qt.insert(elm0_id, elm0_rect);

        println!("\n\ntree:{:?}\n\n", qt);


        let elm1_id = 1.0;
        let elm1_rect = QuadRect::new(QuadPoint {x: -10, y: -10}, QuadPoint { x: -20, y: -20} );
        qt.insert(elm1_id, elm1_rect);

        println!("\n\ntree:{:?}\n\n", qt);

        let elm2_id = 2.0;
        let elm2_rect = QuadRect::new(QuadPoint {x: -10, y: -10}, QuadPoint { x: 20, y: 20} );
        qt.insert(elm2_id, elm2_rect);


        println!("\n\ntree:{:?}\n\n", qt);

        let mut points0 = qt.query(&Query::point(15, 15));

        points0.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert!(*points0[0] == 0.0);
        assert!(*points0[1] == 2.0);
        assert!(points0.len() == 2);


        let mut points1 = qt.query(&Query::point(-1,-1));
        points1.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert!(*points1[0] == 1.0);
        assert!(*points1[1] == 2.0);
        assert!(points1.len() == 2);

        let mut points2 = qt.query(&Query::point(-1, 1));
        points2.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert!(*points2[0] == 2.0);
        assert!(points2.len() == 1);

    }


    #[test]
    fn insert_neg_50_50_elm() {

        let rect = QuadRect::new(QuadPoint {x: -128, y: -128}, QuadPoint { x: 128, y: 128} );

        let mut qt = QuadTree::<(i32, i32)>::new(rect);


        for i in (-51..49).step_by(2) {
            for j in (-51..49).step_by(2) {
                let rect = QuadRect::new(QuadPoint {x: i, y: j}, QuadPoint { x: i, y: j } );
                qt.insert((i,j), rect);
            }
        }

        let points15_15 = qt.query(&Query::point(15, 15));

        println!("{:?}", points15_15);

        assert!(points15_15.len() == 1);

        let points0_0 = qt.query(&Query::point(0, 0));
        assert!(points0_0.len() == 4);

        let search_rect = QuadRect::new(QuadPoint {x: -10, y: -10}, QuadPoint { x: 10, y: 10} );
        let points = qt.query(&Query::rect(search_rect));

        assert!(points.len() == 144)
    }
}
