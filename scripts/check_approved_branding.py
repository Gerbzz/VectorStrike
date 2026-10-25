"""Approved 2026-10-25 branding contract. Requires Pillow; does not modify files."""
from pathlib import Path
from hashlib import sha256
import json
import unittest
from PIL import Image
ROOT = Path(__file__).resolve().parents[1]
APP = 'VectorStrike'
APPROVED = '0b91db169a13a373b820aee362a03bf3958b3bec569dba84c3a8eff230b5aff0'
class ApprovedBranding(unittest.TestCase):
    def test_exact_approved_source_and_readme(self):
        p = ROOT / 'assets/branding' / (APP + '.png')
        self.assertEqual(sha256(p.read_bytes()).hexdigest(), APPROVED)
        with Image.open(p) as image:
            self.assertEqual(image.size, (512,512))
        self.assertIn('assets/branding/' + APP + '.png', (ROOT/'README.md').read_text(encoding="utf-8"))
        self.assertEqual(json.loads((p.parent/'provenance.json').read_text(encoding="utf-8"))['sha256'], APPROVED)
    def test_runtime_derivative_and_binding(self):
        source = Image.open(ROOT/'assets/branding'/(APP+'.png')).convert('RGBA')
        raw = ROOT/'assets/branding'/(APP+'-128.rgba')
        self.assertTrue(raw.exists(), 'runtime derivative absent')
        self.assertEqual(raw.read_bytes(), source.resize((128,128),Image.Resampling.LANCZOS).tobytes())
        self.assertIn(APP+'-128.rgba', (ROOT/'crates/ui-egui/src/brand.rs').read_text(encoding="utf-8"))
    def test_windows_resource_binding(self):
        self.assertIn('set_icon("../../assets/branding/VectorStrike.ico")', (ROOT/'apps/vectorcraft/build.rs').read_text(encoding="utf-8"))

    def test_all_windows_frames_preserve_full_canvas(self):
        from io import BytesIO
        import struct
        data = (ROOT/'assets/branding/VectorStrike.ico').read_bytes()
        sizes = (16,24,32,48,64,128,256)
        self.assertEqual(struct.unpack_from('<HHH', data), (0,1,len(sizes)))
        source = Image.open(ROOT/'assets/branding/VectorStrike.png').convert('RGBA')
        for i,size in enumerate(sizes):
            w,h,c,r,planes,bpp,length,offset = struct.unpack_from('<BBBBHHII',data,6+16*i)
            frame = Image.open(BytesIO(data[offset:offset+length])).convert('RGBA')
            self.assertEqual(frame.size,(size,size))
            self.assertEqual(frame.tobytes(),source.resize((size,size),Image.Resampling.LANCZOS).tobytes())

if __name__ == "__main__":
    unittest.main()
