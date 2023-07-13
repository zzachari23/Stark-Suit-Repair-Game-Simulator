use std::cmp::Ordering;
use std::collections::HashMap;

pub trait PriorityQueue<T: PartialOrd> {
    fn enqueue(&mut self, ele: T) -> ();
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}

/**
    An optional definition of a Node struct you may find useful
**/
struct Node<T> {
    priority: i32,
    data: T,
}

/** 
    These traits are implemented for Nodes to make them comparable 
**/
impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.priority == other.priority
    }
}

fn parent(i: i32) -> i32    { (i-1)/2 }
fn leftchild(i: i32)-> i32  { (2*i)+1 }
fn rightchild(i: i32)-> i32 { (2*i)+2 }



/** 
    You must implement the above trait for the vector type 
**/
impl<T: PartialOrd> PriorityQueue<T> for Vec<T> {
    /**
        This functions pushes a given element onto the queue and
        reorders the queue such that the min heap property holds.
        See the project specifications for more details on how this
        works.
    **/
    fn enqueue(&mut self, ele: T) -> () {
    

        self.push(ele);
        let mut lastposition = self.len() - 1;
            
        while lastposition > 0 && (self[(lastposition - 1)/2] > self[lastposition]) 
        {
            self.swap(lastposition, (lastposition - 1)/2);
            lastposition = (lastposition-1)/2;
        }
    
    }
    
    /**
        This function removes the root element from the queue and
        reorders the queue such that it maintains the min heap
        property.  See the project specifications for more details.
        You should return the deleted element in the form of an option.
        Return None if the queue was initially empty, Some(T) otherwise.
    **/
    fn dequeue(&mut self) -> Option<T> {
       
      if self.len() == 0 {None}
      else
      {
        //Swap the root and last element 
        let mut lastposition = self.len() - 1;
        self.swap(0, lastposition);
        //Pop the root element 
        let root_element = self.pop();
        //Reordering Heap
        let mut index = 0;
        while((2 * index + 1) < self.len())
        {
            let mut small_child_index = 2 * index + 1;
            if (2 * index + 2) < self.len() && (self[2 * index + 2] < self[2 * index + 1])
            {
                small_child_index = 2 * index + 2;
            }

            if self[index] < self[small_child_index]
            {
                break;
            }
            else
            {
                self.swap(index, small_child_index);         
            }
            index = small_child_index;
        }
        root_element

    }
}

    /**
        This function returns the element that would be removed
        if dequeue were called on the queue.  There should be no
        mutations to the queue.  Return the element in the form
        of an option.  Return None if the queue is empty, Some(T)
        otherwise.
    **/
    fn peek(&self) -> Option<&T> {
        
        if self.len() != 0 
        {
            self.get(0)
        }
        else
        {
            None
        }

    }
}


/**
    You must implement this function that computes the orthogonal
    distance between two coordinates.  Remember, orthogonal distance
    is not like Euclidean distance.  See the specifications for more
    details.
**/
pub fn distance(p1: (i32,i32), p2: (i32,i32)) -> i32 {
    let (x1,y1) = p1;
    let (x2,y2) = p2;
    let x = x1 - x2;
    let y = y1 - y2;
    x.abs() + y.abs()

}

/**
    You must implement this function that determines which enemy Stark
    should battle and their coordinates.  You are given two hashmaps for
    allies and enemies.  Each maps a name to their current coordinates.
    You can assume that the allies hashmap will always have a name
    called "Stark" included.  Return the name and coordinates of the enemy
    Stark will battle in the form of a 3-tuple.  See the specifications
    for more details on how to choose which enemy.
**/
pub fn target_locator<'a>(allies: &'a HashMap<&String, (i32,i32)>, enemies: &'a HashMap<&String, (i32,i32)>) -> (&'a str,i32,i32) {

    
    let mut r_value = ("car", 4, 1);
 
    for (enemy_key, enemy_value) in enemies {

        let mut q = Vec::new();

        for (ally_key, ally_value) in allies
        {
            let dist = distance(*ally_value, *enemy_value);
            q.enqueue(dist);
            
        }

        for (ally_key, ally_value) in allies
        {
            let dist = distance(*ally_value, *enemy_value);
            let smallest_distance = q[0];
        
            if dist == smallest_distance && **ally_key == "Stark".to_string()
            {    
                let (x,y) = enemy_value;
                r_value = (enemy_key, *x, *y);
                break;
            }
                            
        }
 
    }

    r_value
}


