class Node {
  constructor(prev, next, shortcut, dht) {
    this.prev = prev || null; 
    this.next = next || null;
    this.shortcut = shortcut || null;
    this.dht = dht;
  }

  addData() {

  }

  queryDht() {

  }

  updateNodes() {
    
  }
}

module.exports = Node;