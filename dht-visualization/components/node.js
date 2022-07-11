class Node {
  constructor(prev, next, shortcut, range) {
    this.prev = prev || null; 
    this.next = next || null;
    this.shortcut = shortcut || null;
    this.min = range[0];
    this.max = range[1];
  }
  
  range() {
    return [this.min, this.max];
  }

  addData() {

  }

  queryDht() {

  }

  updateNodes() {
    
  }
}

module.exports = Node;