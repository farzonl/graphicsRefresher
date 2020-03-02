
public class Vec3 {

    private var slots: [Double] = [0,0,0]

     public init(s0 : Double, s1 : Double, s2 : Double) {
         (self.slots[0], self.slots[1], self.slots[2]) = (s0,s1,s2)
     }
     public var r : Double { return slots[0] }
     public var g : Double { return slots[1] }
     public var b : Double { return slots[2] }

     public var x : Double { return slots[0] }
     public var y : Double { return slots[1] }
     public var z : Double { return slots[2] }

    // Vec3 index Operator
    subscript(index : Int) -> Double {
        get {
            return slots[index]
        }
        set(value) {
            slots.insert(value, at: index)
        }
    }
    // Vec3 Arithmetic Operators
    static func + (l: Vec3, r: Vec3)   -> Vec3 { return Vec3(s0: l.x + r.x, s1: l.y + r.y, s2: l.z + r.z) }
    static prefix func - (l: Vec3)     -> Vec3 { return Vec3(s0: -l.x     , s1: -l.y,      s2: -l.z) }
    static func - (l: Vec3, r: Vec3)   -> Vec3 { return Vec3(s0: l.x - r.x, s1: l.y - r.y, s2: l.z - r.z) }
    static func * (l: Vec3, r: Vec3)   -> Vec3 { return Vec3(s0: l.x * r.x, s1: l.y * r.y, s2: l.z * r.z) }
    static func * (s: Double, v: Vec3) -> Vec3 { return Vec3(s0: s * v.x  , s1: s * v.y,   s2: s * v.z) }
    static func * (v: Vec3, s: Double) -> Vec3 { return Vec3(s0: v.x * s  , s1: v.y * s,   s2: v.z * s) }
    static func / (l: Vec3, r: Vec3)   -> Vec3 { return Vec3(s0: l.x / r.x, s1: l.y / r.y, s2: l.z / r.z) }
    static func / (v: Vec3, s: Double) -> Vec3 { return Vec3(s0: v.x / s  , s1: v.y / s,   s2: v.z / s) }

    // Vec3 Assignment Operators
    static func += (l: inout Vec3, r: Vec3  ) { l = l + r }
    static func -= (l: inout Vec3, r: Vec3  ) { l = l - r }
    static func *= (l: inout Vec3, r: Vec3  ) { l = l * r }
    static func /= (l: inout Vec3, r: Vec3  ) { l = l / r }
    static func *= (l: inout Vec3, r: Double) { l = l * r }
    static func /= (l: inout Vec3, r: Double) { l = l / r }
}