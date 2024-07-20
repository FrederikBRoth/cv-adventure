export default {
  afterRelease: async ({ exec, nextVersion }) => {
    await exec(`test=true`);
  },
  beforePrepare: async ({ exec, nextVersion }) => {
    await exec(`echo "testinglmap" > file.txt`);
  },

};

