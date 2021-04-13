class ParkingLot {
    // Add the methods here
    constructor(slots) {
        // this.slots = Array.from({length: slots}, (_, i) => i + 1);
        this.slots = Array.from({length: slots}, (_) => null);
    }
    
    park(carId) {
        for (let i = 0; i < this.slots.length; i++) {
            if (this.slots[i] === null) {
                this.slots[i] = carId;
                break;
            }
            if (this.slots[i] !== null && i !== this.slots.length - 1) {
                continue;
            }
            return false;
        }
        
        return true;
    }
    
    getSlots() {
        return this.slots;
    }
    
    remove(carId) {
        const sf = this.slots.find(sf => sf === carId);
        if (!sf) {
            return false;
        }
        const idx = this.slots.findIndex(sfi => sfi === sf);
        this.slots[idx] = null;
        return true;
    }
}

const parking = new ParkingLot(5);
console.log(parking.park("car1"));
console.log(parking.park("car2"));
console.log(parking.park("car3"));
console.log(parking.park("car4"));
console.log(parking.park("car5"));
console.log(parking.park("car6"));
console.log(parking.remove("car6"));

console.log(parking.getSlots())
