use std::slice;

struct ChannelBuffer {
    buf: blip_buf::BlipBuf,
    prev_sample: i32,
    frame_size: u32,
}

const BUFFER_SIZE: u32 = 0x10000;

impl ChannelBuffer {
    fn new(clock: f64, sample_rate: f64) -> Self {
        let mut buffer = blip_buf::BlipBuf::new(BUFFER_SIZE);
        buffer.set_rates(clock, sample_rate);
        buffer.clear();

        ChannelBuffer {
            buf: buffer,
            prev_sample: 0,
            frame_size: 0,
        }
    }

    fn push(&mut self, sample: i32) {
        self.buf
            .add_delta(self.frame_size, sample - self.prev_sample);
        self.prev_sample = sample;
        self.frame_size += 1;
    }

    fn end_frame(&mut self) {
        self.buf.end_frame(self.frame_size);
        self.frame_size = 0;
    }
}

pub(crate) struct TwoChannelBuffer {
    left: ChannelBuffer,
    right: ChannelBuffer,
}

impl TwoChannelBuffer {
    pub(crate) fn new(clock: f64, sample_rate: f64) -> Self {
        TwoChannelBuffer {
            left: ChannelBuffer::new(clock, sample_rate),
            right: ChannelBuffer::new(clock, sample_rate),
        }
    }

    pub(crate) fn needed_frame_size(&self, sample_count: usize) -> usize {
        self.left
            .buf
            .clocks_needed(sample_count.try_into().unwrap())
            .try_into()
            .unwrap()
    }

    pub(crate) fn available_sample_count(&self) -> usize {
        self.left.buf.samples_avail().try_into().unwrap()
    }

    pub(crate) fn push(&mut self, left_sample: i32, right_sample: i32) {
        self.left.push(left_sample);
        self.right.push(right_sample);
    }

    pub(crate) fn end_frame(&mut self) {
        self.left.end_frame();
        self.right.end_frame();
    }

    pub(crate) fn pop(&mut self, left_ptr: *mut i16, right_ptr: *mut i16, count: usize) -> usize {
        let (left_buf, right_buf) = unsafe {
            (
                slice::from_raw_parts_mut(left_ptr, count),
                slice::from_raw_parts_mut(right_ptr, count),
            )
        };

        let left_count = self.left.buf.read_samples(left_buf, false);
        let right_count = self.right.buf.read_samples(right_buf, false);

        left_count.min(right_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_buffer() {
        let mut buf = ChannelBuffer::new(8_000_000.0, 44_100.0);
        assert_eq!(buf.frame_size, 0);
        assert_eq!(buf.prev_sample, 0);

        for i in 1..=10 {
            buf.push(i);
            assert_eq!(buf.frame_size, i as u32);
            assert_eq!(buf.prev_sample, i);
        }

        buf.end_frame();
        assert_eq!(buf.frame_size, 0);
        assert_eq!(buf.prev_sample, 10);
    }

    #[test]
    fn test_two_channel_buffer() {
        let mut buf = TwoChannelBuffer::new(8_000_000.0, 44_100.0);

        assert_eq!(buf.available_sample_count(), 0);

        const SAMPLE_COUNT: usize = 1000;
        let size = buf.needed_frame_size(SAMPLE_COUNT);
        for _ in 0..size {
            buf.push(100, -100);
        }
        buf.end_frame();

        assert_eq!(buf.available_sample_count(), SAMPLE_COUNT);

        let (mut left, mut right) = ([0; SAMPLE_COUNT], [0; SAMPLE_COUNT]);
        let actual_count = buf.pop(left.as_mut_ptr(), right.as_mut_ptr(), SAMPLE_COUNT);
        assert_eq!(actual_count, SAMPLE_COUNT);
        assert_eq!(buf.available_sample_count(), 0);
    }
}
