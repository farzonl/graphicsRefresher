let (nx, ny) = (200, 100)
print("P3\n \(nx) \(ny) \n255")
for j in stride(from: ny, to: 0, by: -1) {
  for i in 0...nx-1 {
    let (r,g,b) = ( Double(i) / Double(nx), Double(j) / Double(ny), 0.2)
    let (ir,ig,ib) = ( Int(255.99 * r), Int(255.99 * g), Int(255.99 * b))
    print("\(ir) \(ig) \(ib)")
  }
}
