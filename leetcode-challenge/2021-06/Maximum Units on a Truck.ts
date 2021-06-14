// just sort
function maximumUnits(boxTypes: number[][], truckSize: number): number {
    boxTypes.sort((a, b) => b[1] - a[1]);
    
    let size = truckSize;
    let result = 0;
    for (let i = 0; size > 0 && i < boxTypes.length; i++) {
        const takeSize = Math.min(boxTypes[i][0], size);
        result += takeSize * boxTypes[i][1];
        size -= takeSize;
    }
    
    return result;
};
