class Post {
  constructor(post, timestampOrDate) {
    this.post = post;
    if (typeof(timestampOrDate) === "number") {
      this.timestamp = timestampOrDate;
    } else {
      this.timestamp = Number(timestampOrDate); // convert date to timestampt
    }
  }
  print() {
    const timestampToStrDate = (timestamp) => {
      let isoDateStr = (new Date(timestamp)).toISOString();
      isoDateStr = isoDateStr.replace("T","_");    // format "T"
      isoDateStr = isoDateStr.split(".")[0];       // remove milliseconds
      isoDateStr = isoDateStr.split(":").join(""); // remove colons hh:mm:ss => hhmmss
      return isoDateStr;
    }
    const isoDateStr = timestampToStrDate(this.timestamp);
    console.log(`[${isoDateStr} ${this.timestamp}] ${this.post}`);
  }
}

module.exports = Post;
