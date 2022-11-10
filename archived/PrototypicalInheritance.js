function Activity(amount) {
    this.amount = amount;
};

Activity.prototype.setAmount = function(value) {
    if (value <= 0) {
        return false;
    }
    this.amount = value;
    return true;
};

Activity.prototype.getAmount = function() {
    return this.amount;
};


function Payment(amount, receiver) {
    Activity.call(this, amount);
    this.receiver = receiver;
}
Payment.prototype.setReceiver = function(value) {
    this.receiver = value;
}
Payment.prototype.getReceiver = function() {
    return this.receiver;
}

function Refund(amount, sender) {
    Activity.call(this, amount);
    this.sender = sender;
}
Refund.prototype.setSender = function(value) {
    this.sender = value;
}
Refund.prototype.getSender = function() {
    return this.sender;
}

function extend(base, sub) {
    // Avoid instantiating the base class just to setup inheritance
    // Also, do a recursive merge of two prototypes, so we don't overwrite 
    // the existing prototype, but still maintain the inheritance chain
    // Thanks to @ccnokes
    var origProto = sub.prototype;
    sub.prototype = Object.create(base.prototype);
    for (var key in origProto)  {
       sub.prototype[key] = origProto[key];
    }
    // The constructor property was set wrong, let's fix it
    Object.defineProperty(sub.prototype, 'constructor', { 
      enumerable: false, 
      value: sub 
    });
  }

extend(Activity, Payment);
extend(Activity, Refund);
let payment = new Payment(12, "receiver");
console.log(payment instanceof Activity);

// console.log(payment.setReceiver(14))
// console.log(payment.getReceiver())