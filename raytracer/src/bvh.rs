use crate::aabb::Aabb;
use crate::matirial::Lambertian;
use crate::moving_sphere::MovingSphere;
use crate::rtweekend;
use crate::rtweekend::random_int_a_b;
use crate::HittableList;
use crate::Vec3;
use crate::RAY;
use crate::RAY::Ray;
use std::sync::Arc;
use RAY::{HitRecord, Hittable, Sphere};

pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub hezi: Aabb,
}

impl BvhNode {
    pub fn new_zero() -> BvhNode {
        BvhNode {
            //Arc空指针怎么弄？或者说，RUST就不允许空指针的存在？
            left: Arc::new(Sphere::new_zero()),
            right: Arc::new(Sphere::new_zero()),
            hezi: Aabb::new_zero(),
        }
    }

    pub fn new_dog(list: &HittableList, time0: f64, time1: f64) -> BvhNode {
        BvhNode::new_pig(&list.objects, 0, list.objects.len(), time0, time1)
    }

    pub fn new_pig(
        src_objects: &Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BvhNode {
        let mut objects = src_objects.clone();
        let object_span = end - start;

        let axis = random_int_a_b(0, 2);
        let mut compare: fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> bool =
            BvhNode::box_x_compare;
        if axis == 0 {
            compare = BvhNode::box_x_compare;
        }
        if axis == 1 {
            compare = BvhNode::box_y_compare;
        }
        if axis == 2 {
            compare = BvhNode::box_z_compare;
        }
        let mut left: Arc<dyn Hittable> = Arc::new(Sphere::new_zero());
        let mut right: Arc<dyn Hittable> = Arc::new(Sphere::new_zero());
        if object_span == 1 {
            left = objects[(start)].clone();
            right = objects[(start)].clone();
        } else if object_span == 2 {
            if compare(&objects[(start)], &objects[(start + 1)]) {
                left = objects[(start)].clone();
                right = objects[(start + 1)].clone();
            } else {
                left = objects[(start + 1)].clone();
                right = objects[(start)].clone();
            }
        } else {
            //objects.sort_by(compare);
            objects.sort_by(|a, b| {
                let mut out_put_box = Aabb::new_zero();
                a.bounding_box(time0, time1, &mut out_put_box);
                let pig1 = Some(out_put_box);
                let x = pig1.unwrap().minimum.get_xyz(axis);
                let mut out_put_box2 = Aabb::new_zero();
                b.bounding_box(time0, time1, &mut out_put_box2);
                let pig2 = Some(out_put_box2);
                let y = pig2.unwrap().minimum.get_xyz(axis);
                x.partial_cmp(&y).unwrap()
            });

            let mid = start + object_span / 2;
            left = Arc::new(BvhNode::new_pig(&objects, start, mid, time0, time1));
            right = Arc::new(BvhNode::new_pig(&objects, mid, end, time0, time1));
        }
        let mut box_l = Aabb::new_zero();
        let mut box_r = Aabb::new_zero();
        //println!("kuqiba{}", object_span);
        if !(left.bounding_box(time0, time1, &mut box_l))
            || !(right.bounding_box(time0, time1, &mut box_r))
        {
            println!("NO BOUNDING BOX IN BVH_NODE CONSTRUCT");
        }

        //println!("box_l=={:?} and {:?}", box_l.minimum, box_l.maximum);
        //println!("box_r.x=={} and {}", box_r.minimum.x, box_r.maximum.x);

        let her = MovingSphere::surrounding_box(&box_l, &box_r);
        //println!("her=={:?} and {:?}", her.minimum, her.maximum);
        BvhNode {
            left,
            right,
            hezi: her,
        }
    }
}

impl BvhNode {
    pub fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: i32) -> bool {
        let mut box_a = Aabb::new_zero();
        let mut box_b = Aabb::new_zero();
        if !(a.bounding_box(0.0, 0.0, &mut box_a)) || !(b.bounding_box(0.0, 0.0, &mut box_b)) {
            println!("No bounding box in bvh_node constructor.\n")
        }
        box_a.minimum.get_xyz(axis) < box_b.minimum.get_xyz(axis)
    }

    pub fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
        BvhNode::box_compare(&a, &b, 0)
    }

    pub fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
        BvhNode::box_compare(&a, &b, 1)
    }

    pub fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
        BvhNode::box_compare(&a, &b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        //这里的rec不清楚要不要先赋值
        if !(self.hezi.hit(r, t_min, t_max)) {
            // println!("1");
            return false;
        }

        let hit_left = self.left.hit(r, t_min, t_max, rec); //这里要引用吗？不清楚
        let hit_right: bool;
        if hit_left {
            hit_right = self.right.hit(r, t_min, rec.t, rec)
        } else {
            hit_right = self.right.hit(r, t_min, t_max, rec);
        }

        if (hit_left || hit_right) {
            return true;
        }
        //println!("2");
        false
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        output_box.maximum = self.hezi.maximum.clone();
        output_box.minimum = self.hezi.minimum.clone();
        true
    }
}
