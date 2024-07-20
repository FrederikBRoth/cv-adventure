export default {
  afterRelease: async ({ exec, nextVersion }) => {
    await exec(`gh release upload ${nextVersion}`);
  },
};

