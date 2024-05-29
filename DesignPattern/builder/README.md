# 建造者模式
Rust的建造者模式与Java一致，通过链式调用初始化类(结构体)实例。

建造者模式的用途在哪？个人感觉是在构建复杂对象实例的时候，可以针对某个属性进行修改而其他属性保持默认值。比如说工厂在构建机器人时，默认的机器人`Robot::default()`就可以出厂了。当定制化机器人时`Robot::default().core(8)`可以保持其他配置不变，只修改某个属性即可。
```rust
fn main() {
    Table::default()
        .column_spacing(1)
        .widths([
            Constraint::Length(15),
            Constraint::Min(20),
            Constraint::Length(15),
        ])
        .style(Style::new().blue())
        .highlight_style(Style::new().reversed())
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_symbol(">>")
        .rows(table_items);
}
```

## 实现
```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn default() ->&mut Circle {
        // 默认值
        &mut Circle {
            x: 0.0, y: 0.0, radius: 1.0,
        }
    }
    fn x(&mut self, coordinate: f64) -> &mut Circle {
        self.x = coordinate;
        self
    }
    fn y(&mut self, coordinate: f64) -> &mut Circle {
        self.y = coordinate;
        self
    }
    fn radius(&mut self, radius: f64) -> &mut Circle {
        self.radius = radius;
        self
    }
}

fn main() {
  let c = Circle::default().x(1.0).y(2.0).radius(2.0);
  println!("area = {:?}", c.area());
  println!("c.x = {:?}", c.x);
  println!("c.y = {:?}", c.y);
}
```
由于Rust本身没有重载，在实现复杂对象构造函数时，这种方式的可读性会更好。