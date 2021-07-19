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

    pub fn assignment_2(&mut self, list: &HittableList, time0: f64, time1: f64) {
        self.assignment(&list.objects, 0, list.objects.len(), time0, time1);
    }

    pub fn assignment(
        &mut self,
        src_objects: &Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) {
        let mut objects = src_objects.clone();
        let axis = random_int_a_b(0, 2);
        let mut compare: fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> bool;
        //if axis==0 {
        compare = BvhNode::box_x_compare;
        if axis == 1 {
            compare = BvhNode::box_y_compare;
        }
        if axis == 2 {
            compare = BvhNode::box_z_compare;
        }
        let object_span = end - start;

        if object_span == 1 {
            self.left = objects[(start)].clone();
            self.right = objects[(start)].clone();
        } else if object_span == 2 {
            if compare(&objects[(start)], &objects[(start + 1)]) {
                self.left = objects[(start)].clone();
                self.right = objects[(start + 1)].clone();
            } else {
                self.left = objects[(start + 1)].clone();
                self.right = objects[(start)].clone();
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
                let y = pig2.unwrap().maximum.get_xyz(axis);
                x.partial_cmp(&y).unwrap()
            });

            let mid = start + object_span / 2;
            let mut B1 = BvhNode::new_zero();
            B1.assignment(&objects, start, mid, time0, time1);
            self.left = Arc::new(B1);
            let mut B2 = BvhNode::new_zero();
            B2.assignment(&objects, mid, end, time0, time1);
            self.right = Arc::new(B2);
        }
        let mut box_l = Aabb::new_zero();
        let mut box_r = Aabb::new_zero();
        if !(self.left.bounding_box(time0, time1, &mut box_l))
            || !(self.right.bounding_box(time0, time1, &mut box_r))
        {
            println!("NO BOUNDING BOX IN BVH_NODE CONSTRUCT");
        }
        self.hezi = MovingSphere::surrounding_box(&box_l, &box_r);
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
        if !(self.hezi.hit(&r, t_min, t_max)) {
            return false;
        }
        let mut rec_mid = HitRecord::new_blank();
        let hit_left = self.left.hit(&r, t_min, t_max, &mut rec_mid); //这里要引用吗？不清楚
        let hit_right: bool;
        if hit_left {
            hit_right = self.right.hit(&r, t_min, rec_mid.t, &mut rec_mid)
        } else {
            hit_right = self.right.hit(&r, t_min, t_max, &mut rec_mid);
        }
        rec.front_face = rec_mid.front_face;
        rec.t = rec_mid.t;
        rec.p = rec_mid.p.clone();
        rec.mat_ptr = rec_mid.mat_ptr.clone();
        rec.normal = rec_mid.normal.clone();
        hit_left || hit_right
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        output_box.maximum = self.hezi.maximum.clone();
        output_box.minimum = self.hezi.minimum.clone();
        true
    }
}
