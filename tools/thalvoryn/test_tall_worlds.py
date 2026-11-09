import importlib.util
import pathlib
import tempfile
import unittest

PATH = pathlib.Path(__file__).with_name('patch_horizon_tall_worlds.py')
spec = importlib.util.spec_from_file_location('tallworlds', PATH)
tw = importlib.util.module_from_spec(spec)
spec.loader.exec_module(tw)

PORTAL_SRC = f'''// synthetic pinned-code excerpt
{tw.SECTIONS_OLD}
let q = true;
{tw.SECTIONS_OLD}
{tw.LOGICAL_OLD}
'''
RAIDS_SRC = '''impl Game<'_> {
    pub(super) fn surface_height(&self, x: i32, z: i32) -> i32 {
        let top = if self.dim == Dim::Nether { 127 } else { 319 };
        (self.world.bottom_y()..=top).rev().find(|&y| y>0);
    }
}'''

class TestTallWorlds(unittest.TestCase):
    def test_portal_exact(self):
        out = tw.patch_portal(PORTAL_SRC)
        self.assertEqual(out.count(tw.SECTIONS_NEW), 2)
        self.assertEqual(out.count(tw.LOGICAL_NEW), 1)
    def test_portal_reject_wrong(self):
        with self.assertRaises(ValueError): tw.patch_portal(PORTAL_SRC.replace(tw.LOGICAL_OLD, 'bad'))
    def test_portal_idempotence(self):
        with self.assertRaises(ValueError): tw.patch_portal(tw.patch_portal(PORTAL_SRC))
    def test_raids_exact(self):
        out = tw.patch_raids(RAIDS_SRC)
        self.assertEqual(out.count(tw.RAIDS_NEW), 1)
    def test_raids_reject_wrong(self):
        with self.assertRaises(ValueError): tw.patch_raids('x' + tw.RAIDS_OLD)
    def test_raids_idempotence(self):
        with self.assertRaises(ValueError): tw.patch_raids(tw.patch_raids(RAIDS_SRC))
    def test_vanilla_nether(self):
        self.assertEqual(tw.profile(0,256,True), {'min_section':0,'max_section':15,'portal_logical_height':128,'raid_surface_top':127})
    def test_vanilla_overworld(self):
        self.assertEqual(tw.profile(-64,384,False), {'min_section':-4,'max_section':19,'portal_logical_height':384,'raid_surface_top':319})
    def test_nether_1k(self):
        self.assertEqual(tw.profile(0,1024,True), {'min_section':0,'max_section':63,'portal_logical_height':1024,'raid_surface_top':1023})
    def test_nether_2k(self):
        self.assertEqual(tw.profile(0,2032,True), {'min_section':0,'max_section':126,'portal_logical_height':2032,'raid_surface_top':2031})
    def test_overworld_tall(self):
        self.assertEqual(tw.profile(-64,2096,False), {'min_section':-4,'max_section':126,'portal_logical_height':2096,'raid_surface_top':2031})
    def test_bad_alignment(self):
        with self.assertRaises(ValueError): tw.profile(1,1024,True)
    def checkout(self, path):
        p = pathlib.Path(path)
        for rel, text in ((tw.PORTAL,PORTAL_SRC),(tw.RAIDS,RAIDS_SRC)):
            f=p/rel;f.parent.mkdir(parents=True,exist_ok=True);f.write_text(text)
    def test_dry_run(self):
        with tempfile.TemporaryDirectory() as root:
            self.checkout(root)
            self.assertEqual(tw.main([root]),0)
            self.assertEqual((pathlib.Path(root)/tw.PORTAL).read_text(),PORTAL_SRC)
    def test_apply_with_backups(self):
        with tempfile.TemporaryDirectory() as root:
            self.checkout(root)
            self.assertEqual(tw.main([root,'--apply']),0)
            self.assertEqual((pathlib.Path(root)/tw.PORTAL).with_name(tw.PORTAL.name + '.thalvoryn-original.bak').read_text(),PORTAL_SRC)
            self.assertEqual((pathlib.Path(root)/tw.RAIDS).with_name(tw.RAIDS.name + '.thalvoryn-original.bak').read_text(),RAIDS_SRC)
    def test_repeat_rejected_without_mutation(self):
        with tempfile.TemporaryDirectory() as root:
            self.checkout(root)
            self.assertEqual(tw.main([root,'--apply']),0)
            before=(pathlib.Path(root)/tw.PORTAL).read_bytes()
            self.assertEqual(tw.main([root,'--apply']),2)
            self.assertEqual((pathlib.Path(root)/tw.PORTAL).read_bytes(),before)
    def test_unexpected_missing_file_atomic(self):
        with tempfile.TemporaryDirectory() as root:
            self.checkout(root)
            (pathlib.Path(root)/tw.RAIDS).unlink()
            self.assertEqual(tw.main([root,'--apply']),2)
            self.assertFalse((pathlib.Path(root)/tw.PORTAL).with_name(tw.PORTAL.name+'.thalvoryn-original.bak').exists())

if __name__=='__main__': unittest.main(verbosity=2)