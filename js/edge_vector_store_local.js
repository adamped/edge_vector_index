class Index {
    constructor(vectors, metadata) {
      this.vectors = vectors;
      this.metadata = metadata;
    }
  
    get Vectors() {
      return this.vectors;
    }
  
    get Metadata() {
      return this.metadata;
    }
  }

class EdgeVectorIndexLocal {
    constructor() {
      this.index = [];
    }
  
    addToIndex(initialData) {
      this.index.push(initialData);
    }
  
    findClosestMatch(vector) {
      let cosine = 0.0;
      let closestMatch;
  
      for (const item of this.index) {
        const similarity = this.cosineSimilarity(vector, item.Vectors);
        if (similarity >= cosine) {
          cosine = similarity;
          closestMatch = item;
        }
      }
  
      return closestMatch;
    }
  
    dotProductCalc(a, b) {
      return a.map((x, i) => x * b[i]).reduce((sum, element) => sum + element, 0.0);
    }
  
    cosineSimilarity(vector1, vector2) {
      if (vector1.length !== vector2.length) {
        throw new Error("Vectors must have the same length");
      }
  
      const dotProduct = this.dotProductCalc(vector1, vector2);
      const sumSq1 = this.dotProductCalc(vector1, vector1);
      const sumSq2 = this.dotProductCalc(vector2, vector2);
  
      const magnitude = Math.sqrt(sumSq1) * Math.sqrt(sumSq2);
  
      if (magnitude === 0.0) {
        // With 0 magnitude the relationship is undefined
        return NaN;
      }
  
      return dotProduct / magnitude;
    }
  }
  