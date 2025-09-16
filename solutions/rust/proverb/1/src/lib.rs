
pub fn build_proverb(list: &[&str]) -> String {
    let n=list.len();
    
    let mut s:String = String::new(); 
    if n==0{
        return s;
    }
    let mut i=0;
    while(i<n-1){
        s+=&format!("For want of a {} the {} was lost.\n",list[i],list[i+1]);
        i+=1;
    }
    s+=&format!("And all for the want of a {}.",list[0]);
    // println!("And all for the want of a {}.",list[0]);
    s
}
