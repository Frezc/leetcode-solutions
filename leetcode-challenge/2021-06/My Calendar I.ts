class MyCalendar {
    private events: Array<{ start: number, end: number }> = [];
    constructor() {
    }

    book(start: number, end: number): boolean {
        let i = 0;
        while (i <= this.events.length) {
            if ((!this.events[i] || this.events[i].start >= end) && (!this.events[i - 1] || this.events[i - 1].end <= start)) {
                this.insert(i, start, end);
                return true;
            }
            i++;
        }
        
        return false;
    }

    private insert(i: number, start: number, end: number) {
        this.events.splice(i, 0, { start, end });
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * var obj = new MyCalendar()
 * var param_1 = obj.book(start,end)
 */
