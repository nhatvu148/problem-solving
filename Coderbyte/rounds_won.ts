export function rounds_won( results ) {
    const scores = {};
    for (let i = 0; i < results.length; i++) {
        const result = results[i];
       scores[result] = scores[result] ? scores[result] + 1 : 1;
    }
    
    const keys = Object.keys(scores);
    const key1 = keys[0];
    
    if (keys.length === 1) {
       return scores[key1]; 
    }
    
    const key2 = keys[1];
    if (scores[key1] > scores[key2]) {
        return scores[key1];
    } else {
        return scores[key2];
    }
}
