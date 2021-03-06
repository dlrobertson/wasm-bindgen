enum ShapeType { "circle", "square" };

[Constructor(ShapeType kind)]
interface Shape {
  static Shape triangle();

  [Pure]
    boolean isSquare();

  [Pure]
    boolean isCircle();

  [Pure]
    ShapeType getShape();
};
